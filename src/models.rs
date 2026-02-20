use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubUser {
    pub login: String,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub avatar_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GithubRepo {
    pub name: String,
    pub stargazers_count: u32,
    pub forks_count: u32,
    pub language: Option<String>,
    pub fork: bool,
}

#[derive(Debug, Clone)]
pub struct UserStats {
    pub login: String,
    pub public_repos: u32,
    pub followers: u32,
    pub following: u32,
    pub total_stars: u32,
    pub total_forks: u32,
}

#[derive(Debug, Clone)]
pub struct LanguageStats {
    pub login: String,
    pub languages: Vec<(String, u32)>, // (language, repo_count) sorted by count desc
}

// GraphQL response types for contributions
#[derive(Debug, Deserialize)]
pub struct GraphQLResponse {
    pub data: Option<GraphQLData>,
    pub errors: Option<Vec<GraphQLError>>,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
pub struct GraphQLData {
    pub user: Option<GraphQLUser>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphQLUser {
    pub contributions_collection: ContributionsCollection,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributionsCollection {
    pub total_commit_contributions: u32,
    pub total_pull_request_contributions: u32,
    pub total_issue_contributions: u32,
    pub total_pull_request_review_contributions: u32,
    pub contribution_calendar: ContributionCalendar,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributionCalendar {
    pub total_contributions: u32,
    pub weeks: Vec<ContributionWeek>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributionWeek {
    pub contribution_days: Vec<ContributionDay>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributionDay {
    pub contribution_count: u32,
    pub date: String,
}

#[derive(Debug, Clone)]
pub struct ContributionStats {
    pub login: String,
    pub total_contributions: u32,
    pub total_commits: u32,
    pub total_prs: u32,
    pub total_issues: u32,
    pub total_reviews: u32,
    pub current_streak: u32,
    pub longest_streak: u32,
}
