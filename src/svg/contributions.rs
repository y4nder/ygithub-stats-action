use crate::models::ContributionStats;
use crate::themes::Theme;
use super::common::{
    CARD_WIDTH, PADDING, COL1_X, COL2_X, TITLE_WIDTH, TITLE_Y, BORDER_LEFT, BORDER_RIGHT, BORDER_TOP,
    escape_xml, counter_css, render_counter, render_ascii_bar,
};

const CONTRIB_CARD_HEIGHT: i32 = 250;

fn streak_indicator(days: u32) -> &'static str {
    match days {
        0 => "○",
        1..=7 => "◐",
        8..=30 => "●",
        31..=100 => "◉",
        _ => "★",
    }
}

fn render_contrib_stat_row(
    x_base: i32,
    y: i32,
    label: &str,
    value: u32,
    max: u32,
    theme: &Theme,
    delay_index: usize,
) -> String {
    let bar_x = x_base + 115;
    let bar_y = y - 1;
    let value_x = x_base + 70;

    let value_counter = render_counter(value_x, y, value, "mono value", delay_index);

    format!(
        r#"<text x="{label_x}" y="{y}" class="mono label">{label}</text>
        {value_counter}
        {bar}"#,
        label_x = x_base,
        y = y,
        label = label,
        value_counter = value_counter,
        bar = render_ascii_bar(
            bar_x,
            bar_y,
            value,
            max,
            60,
            &theme.accent_color,
            &theme.dim_color,
            "⣿",
            "⣀",
            delay_index,
        ),
    )
}

fn render_streak_stat(
    x: i32,
    y: i32,
    indicator: &str,
    label: &str,
    days: u32,
    delay_index: usize,
) -> String {
    let days_counter = render_counter(x + 75, y, days, "mono streak", delay_index);

    format!(
        r#"<text x="{x}" y="{y}" class="mono streak">{indicator}</text>
        <text x="{label_x}" y="{y}" class="mono label">{label}</text>
        {days_counter}
        <text x="{suffix_x}" y="{y}" class="mono streak"> days</text>"#,
        x = x,
        y = y,
        indicator = indicator,
        label_x = x + 18,
        label = label,
        days_counter = days_counter,
        suffix_x = x + 100,
    )
}

pub fn render_contributions_card(stats: &ContributionStats, theme: &Theme) -> String {
    // Explicit Y positions to prevent overlap
    let contrib_header_y = 35;
    let contrib_total_y = 60;

    // Activity section
    let activity_header_y = 85;
    let commits_y = 110;
    let prs_y = 130;

    // Streak section
    let streak_header_y = 160;
    let streak_row_y = 185;
    let activity_bar_y = 210;

    // Footer
    let footer_y = CONTRIB_CARD_HEIGHT - 20;

    // Border calculations
    let border_bottom = CONTRIB_CARD_HEIGHT - 8;
    let border_height = CONTRIB_CARD_HEIGHT - 28;
    let corner_bottom = CONTRIB_CARD_HEIGHT - 10;

    // Build stat rows
    let commits_row = render_contrib_stat_row(COL1_X, commits_y, "COMMITS", stats.total_commits, 1000, theme, 0);
    let prs_row = render_contrib_stat_row(COL1_X, prs_y, "PRS", stats.total_prs, 100, theme, 1);
    let issues_row = render_contrib_stat_row(COL2_X, commits_y, "ISSUES", stats.total_issues, 100, theme, 2);
    let reviews_row = render_contrib_stat_row(COL2_X, prs_y, "REVIEWS", stats.total_reviews, 100, theme, 3);

    // Build streak stats
    let current_streak = render_streak_stat(
        COL1_X,
        streak_row_y,
        streak_indicator(stats.current_streak),
        "CURRENT",
        stats.current_streak,
        5,
    );
    let longest_streak = render_streak_stat(
        COL2_X,
        streak_row_y,
        streak_indicator(stats.longest_streak),
        "LONGEST",
        stats.longest_streak,
        6,
    );

    // Streak activity bar
    let streak_bar = render_ascii_bar(
        COL1_X + 70,
        activity_bar_y - 1,
        stats.current_streak,
        30,
        250,
        "#fb8c00",
        &theme.dim_color,
        "⣿",
        "⣀",
        4,
    );

    // Title bar dimensions
    let title_x = (CARD_WIDTH - TITLE_WIDTH) / 2;
    let center_x = CARD_WIDTH / 2;

    // Header lines
    let contrib_line = "-".repeat(36);
    let activity_line = "-".repeat(41);
    let streak_line = "-".repeat(43);

    // Total contributions counter
    let total_counter = render_counter(COL1_X + 65, contrib_total_y, stats.total_contributions, "mono total", 0);

    format!(
        r##"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" fill="none" xmlns="http://www.w3.org/2000/svg">
    <style>
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&amp;display=swap');
        @keyframes bar-load {{
            from {{ clip-path: inset(0 100% 0 0); }}
            to {{ clip-path: inset(0 0 0 0); }}
        }}
        {counter_css}
        .mono {{ font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace; }}
        .title {{ font-size: 10px; font-weight: 700; fill: {title_color}; }}
        .header {{ font-size: 12px; font-weight: 700; fill: {title_color}; }}
        .label {{ font-size: 11px; fill: {dim_color}; }}
        .value {{ font-size: 11px; font-weight: 700; fill: {text_color}; }}
        .streak {{ font-size: 11px; fill: #fb8c00; }}
        .hint {{ font-size: 9px; fill: {dim_color}; }}
        .key {{ fill: {title_color}; }}
        .total {{ font-size: 16px; font-weight: 700; fill: {accent_color}; }}
        .bar-text {{ font-size: 9px; }}
        .bar-fill {{ fill: {accent_color}; animation: bar-load 1s ease-out forwards; }}
        .bar-empty {{ fill: {dim_color}; opacity: 0.5; }}
    </style>

    <!-- Background -->
    <rect x="1" y="1" width="{bg_width}" height="{bg_height}" rx="0" fill="{bg_color}"/>

    <!-- Left/Right borders -->
    <rect x="{border_left}" y="14" width="1.5" height="{border_height}" fill="{border_color}"/>
    <rect x="{border_right}" y="14" width="1.5" height="{border_height}" fill="{border_color}"/>

    <!-- Top/Bottom borders -->
    <rect x="{border_left}" y="{border_top}" width="404" height="1.5" fill="{border_color}"/>
    <rect x="{border_left}" y="{border_bottom}" width="404" height="1.5" fill="{border_color}"/>

    <!-- Corner pixels -->
    <rect x="6" y="6" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="410" y="6" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="6" y="{corner_bottom}" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="410" y="{corner_bottom}" width="4" height="4" rx="1" fill="{border_color}"/>

    <!-- Title bar -->
    <rect x="{title_x}" y="2" width="{title_width}" height="12" fill="{bg_color}"/>
    <text x="{center_x}" y="{title_y}" text-anchor="middle" class="mono title">┤ {login} ├</text>

    <!-- Contributions Section -->
    <text x="{padding}" y="{contrib_header_y}" class="mono header">- Contributions {contrib_line}</text>

    <text x="{col1}" y="{contrib_total_y}" class="mono label">TOTAL</text>
    {total_counter}
    <text x="{total_desc_x}" y="{contrib_total_y}" class="mono label">
        contributions this year
    </text>

    <!-- Activity Section -->
    <text x="{padding}" y="{activity_header_y}" class="mono header">- Activity {activity_line}</text>

    <!-- Left Column Stats -->
    {commits_row}
    {prs_row}

    <!-- Right Column Stats -->
    {issues_row}
    {reviews_row}

    <!-- Streak Section -->
    <text x="{padding}" y="{streak_header_y}" class="mono header">- Streak {streak_line}</text>

    {current_streak}
    {longest_streak}

    <!-- Streak Visualization -->
    <text x="{col1}" y="{activity_bar_y}" class="mono label">ACTIVITY</text>
    {streak_bar}

    <!-- Footer hint -->
    <text x="{center_x}" y="{footer_y}" text-anchor="middle" class="mono hint">
        <tspan class="key">[C]</tspan> Contrib
        <tspan dx="10" class="key">[S]</tspan> Streak
        <tspan dx="10" class="key">[T]</tspan> Theme
    </text>
</svg>"##,
        width = CARD_WIDTH,
        height = CONTRIB_CARD_HEIGHT,
        bg_width = CARD_WIDTH - 2,
        bg_height = CONTRIB_CARD_HEIGHT - 2,
        bg_color = theme.bg_color,
        border_color = theme.border_color,
        title_color = theme.title_color,
        text_color = theme.text_color,
        dim_color = theme.dim_color,
        accent_color = theme.accent_color,
        counter_css = counter_css(),
        border_left = BORDER_LEFT,
        border_right = BORDER_RIGHT,
        border_top = BORDER_TOP,
        border_bottom = border_bottom,
        border_height = border_height,
        corner_bottom = corner_bottom,
        title_x = title_x,
        title_width = TITLE_WIDTH,
        center_x = center_x,
        title_y = TITLE_Y,
        padding = PADDING,
        col1 = COL1_X,
        total_desc_x = COL1_X + 170,
        contrib_header_y = contrib_header_y,
        contrib_total_y = contrib_total_y,
        activity_header_y = activity_header_y,
        streak_header_y = streak_header_y,
        activity_bar_y = activity_bar_y,
        footer_y = footer_y,
        login = escape_xml(&stats.login),
        total_counter = total_counter,
        commits_row = commits_row,
        prs_row = prs_row,
        issues_row = issues_row,
        reviews_row = reviews_row,
        current_streak = current_streak,
        longest_streak = longest_streak,
        streak_bar = streak_bar,
        contrib_line = contrib_line,
        activity_line = activity_line,
        streak_line = streak_line,
    )
}
