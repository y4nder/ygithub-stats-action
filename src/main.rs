mod github;
mod graphql;
mod models;
mod svg;
mod themes;

fn main() -> anyhow::Result<()> {
    // Read GitHub Action inputs from environment
    let username = std::env::var("INPUT_USERNAME")
        .map_err(|_| anyhow::anyhow!("Missing required input: username"))?;

    let theme_name = std::env::var("INPUT_THEME").unwrap_or_else(|_| "dark".into());
    let card_type = std::env::var("INPUT_CARD").unwrap_or_else(|_| "stats".into());
    let output_path = std::env::var("INPUT_OUTPUT").unwrap_or_else(|_| match card_type.as_str() {
        "languages" => "languages.svg".into(),
        "contributions" => "contributions.svg".into(),
        _ => "stats.svg".into(),
    });

    let token = std::env::var("INPUT_TOKEN")
        .ok()
        .filter(|t| !t.trim().is_empty())
        .or_else(|| {
            std::env::var("GITHUB_TOKEN")
                .ok()
                .filter(|t| !t.trim().is_empty())
        })
        .ok_or_else(|| anyhow::anyhow!("Missing token (INPUT_TOKEN or GITHUB_TOKEN)"))?;

    // Build HTTP client
    let client = reqwest::blocking::Client::builder()
        .user_agent("ygithub-stats-action")
        .build()?;

    let theme = themes::get_theme(&theme_name);

    // Generate the appropriate card based on card_type
    let svg_content = match card_type.to_lowercase().as_str() {
        "languages" | "langs" => {
            eprintln!("Fetching language stats for {}...", username);
            let stats = github::fetch_language_stats(&client, &username, &token)?;
            svg::render_languages_card(&stats, &theme)
        }
        "contributions" | "contrib" => {
            eprintln!("Fetching contribution stats for {}...", username);
            let stats = graphql::fetch_contribution_stats(&client, &username, &token)?;
            svg::render_contributions_card(&stats, &theme)
        }
        _ => {
            eprintln!("Fetching GitHub stats for {}...", username);
            let stats = github::fetch_user_stats(&client, &username, &token)?;
            svg::render_stats_card(&stats, &theme)
        }
    };

    // Write output
    std::fs::write(&output_path, &svg_content)?;
    println!(
        "Generated {} for user {} (card: {}, theme: {})",
        output_path, username, card_type, theme_name
    );

    Ok(())
}
