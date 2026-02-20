use crate::models::{ContributionStats, ContributionWeek, GraphQLResponse};
use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

const CONTRIBUTIONS_QUERY: &str = r#"
query($login: String!) {
  user(login: $login) {
    contributionsCollection {
      totalCommitContributions
      totalPullRequestContributions
      totalIssueContributions
      totalPullRequestReviewContributions
      contributionCalendar {
        totalContributions
        weeks {
          contributionDays {
            contributionCount
            date
          }
        }
      }
    }
  }
}
"#;

fn calculate_streaks(weeks: &[ContributionWeek]) -> (u32, u32) {
    // Flatten all days and sort by date
    let mut days: Vec<_> = weeks
        .iter()
        .flat_map(|w| w.contribution_days.iter())
        .collect();
    days.sort_by(|a, b| a.date.cmp(&b.date));

    let mut current_streak = 0u32;
    let mut longest_streak = 0u32;
    let mut temp_streak = 0u32;

    for day in &days {
        if day.contribution_count > 0 {
            temp_streak += 1;
            longest_streak = longest_streak.max(temp_streak);
        } else {
            temp_streak = 0;
        }
    }

    // Calculate current streak from the end
    for day in days.iter().rev() {
        if day.contribution_count > 0 {
            current_streak += 1;
        } else {
            break;
        }
    }

    (current_streak, longest_streak)
}

pub fn fetch_contribution_stats(
    client: &Client,
    username: &str,
    token: &str,
) -> Result<ContributionStats> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );

    let body = serde_json::json!({
        "query": CONTRIBUTIONS_QUERY,
        "variables": {
            "login": username
        }
    });

    let response = client
        .post("https://api.github.com/graphql")
        .headers(headers)
        .json(&body)
        .send()?
        .error_for_status()?;

    let gql_response: GraphQLResponse = response.json()?;

    if let Some(errors) = gql_response.errors {
        let msg = errors
            .iter()
            .map(|e| e.message.as_str())
            .collect::<Vec<_>>()
            .join(", ");
        return Err(anyhow!("GraphQL errors: {}", msg));
    }

    let user = gql_response
        .data
        .and_then(|d| d.user)
        .ok_or_else(|| anyhow!("User not found: {}", username))?;

    let collection = user.contributions_collection;
    let (current_streak, longest_streak) =
        calculate_streaks(&collection.contribution_calendar.weeks);

    Ok(ContributionStats {
        login: username.to_string(),
        total_contributions: collection.contribution_calendar.total_contributions,
        total_commits: collection.total_commit_contributions,
        total_prs: collection.total_pull_request_contributions,
        total_issues: collection.total_issue_contributions,
        total_reviews: collection.total_pull_request_review_contributions,
        current_streak,
        longest_streak,
    })
}
