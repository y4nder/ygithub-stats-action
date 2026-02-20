use crate::models::{GithubRepo, GithubUser, LanguageStats, UserStats};
use std::collections::HashMap;
use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};

fn auth_headers(token: &str) -> Result<HeaderMap> {
    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("Bearer {}", token))?,
    );
    Ok(headers)
}

pub fn fetch_github_user(client: &Client, username: &str, token: &str) -> Result<GithubUser> {
    let url = format!("https://api.github.com/users/{}", username);
    let response = client
        .get(&url)
        .headers(auth_headers(token)?)
        .send()?
        .error_for_status()?;

    let user = response.json::<GithubUser>()?;
    Ok(user)
}

fn fetch_all_repos(client: &Client, username: &str, token: &str) -> Result<Vec<GithubRepo>> {
    let mut all_repos = Vec::new();
    let mut page = 1;
    let per_page = 100;

    loop {
        let url = format!(
            "https://api.github.com/users/{}/repos?per_page={}&page={}",
            username, per_page, page
        );

        let response = client
            .get(&url)
            .headers(auth_headers(token)?)
            .send()?
            .error_for_status()?;

        let repos: Vec<GithubRepo> = response.json()?;
        let count = repos.len();
        all_repos.extend(repos);

        if count < per_page {
            break;
        }
        page += 1;
    }

    Ok(all_repos)
}

pub fn fetch_user_stats(client: &Client, username: &str, token: &str) -> Result<UserStats> {
    let user = fetch_github_user(client, username, token)?;
    let repos = fetch_all_repos(client, username, token)?;

    // Calculate totals (excluding forks for stars/forks count)
    let (total_stars, total_forks) = repos
        .iter()
        .filter(|r| !r.fork)
        .fold((0, 0), |(stars, forks), repo| {
            (stars + repo.stargazers_count, forks + repo.forks_count)
        });

    let stats = UserStats {
        login: user.login,
        public_repos: user.public_repos,
        followers: user.followers,
        following: user.following,
        total_stars,
        total_forks,
    };

    Ok(stats)
}

pub fn fetch_language_stats(client: &Client, username: &str, token: &str) -> Result<LanguageStats> {
    let user = fetch_github_user(client, username, token)?;
    let repos = fetch_all_repos(client, username, token)?;

    // Count repos per language (excluding forks)
    let mut lang_counts: HashMap<String, u32> = HashMap::new();
    for repo in repos.iter().filter(|r| !r.fork) {
        if let Some(ref lang) = repo.language {
            *lang_counts.entry(lang.clone()).or_insert(0) += 1;
        }
    }

    // Sort by count descending
    let mut languages: Vec<(String, u32)> = lang_counts.into_iter().collect();
    languages.sort_by(|a, b| b.1.cmp(&a.1));

    Ok(LanguageStats {
        login: user.login,
        languages,
    })
}
