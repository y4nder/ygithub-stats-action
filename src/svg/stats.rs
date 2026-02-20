use crate::models::UserStats;
use crate::themes::Theme;
use super::common::{
    CARD_WIDTH, CARD_HEIGHT, PADDING, COL1_X, COL2_X, VALUE_OFFSET, BAR_OFFSET, MAX_BAR_WIDTH,
    TITLE_WIDTH, TITLE_Y, BORDER_LEFT, BORDER_RIGHT, BORDER_TOP,
    escape_xml, counter_css, render_counter, render_ascii_bar,
};

fn render_stat_row(
    y: i32,
    label: &str,
    value: u32,
    max: Option<u32>,
    theme: &Theme,
    delay_index: usize,
) -> String {
    let label_x = COL1_X;
    let value_x = label_x + VALUE_OFFSET;

    let bar_svg = if let Some(max_val) = max {
        let bar_x = label_x + BAR_OFFSET;
        let bar_y = y - 1;
        render_ascii_bar(
            bar_x,
            bar_y,
            value,
            max_val,
            MAX_BAR_WIDTH,
            &theme.accent_color,
            &theme.dim_color,
            "⣿",
            "⣀",
            delay_index,
        )
    } else {
        String::new()
    };

    let value_counter = render_counter(value_x, y, value, "mono value", delay_index);

    format!(
        r#"<text x="{label_x}" y="{y}" class="mono label">{label}</text>
        {value_counter}
        {bar_svg}"#,
        label_x = label_x,
        y = y,
        value_counter = value_counter,
        bar_svg = bar_svg,
    )
}

fn render_double_stat_row(
    y: i32,
    label1: &str,
    value1: u32,
    label2: &str,
    value2: u32,
    delay_index: usize,
) -> String {
    let label1_x = COL1_X;
    let value1_x = label1_x + VALUE_OFFSET;
    let label2_x = COL2_X;
    let value2_x = label2_x + VALUE_OFFSET;

    let counter1 = render_counter(value1_x, y, value1, "mono value", delay_index);
    let counter2 = render_counter(value2_x, y, value2, "mono value", delay_index + 1);

    format!(
        r#"<text x="{label1_x}" y="{y}" class="mono label">{label1}</text>
        {counter1}
        <text x="{label2_x}" y="{y}" class="mono label">{label2}</text>
        {counter2}"#,
        label1_x = label1_x,
        y = y,
        label1 = label1,
        counter1 = counter1,
        label2_x = label2_x,
        label2 = label2,
        counter2 = counter2,
    )
}

pub fn render_stats_card(stats: &UserStats, theme: &Theme) -> String {
    // Explicit Y positions to prevent overlap
    let stats_header_y = 30;
    let stars_y = 48;
    let forks_y = 68;
    let repos_y = 88;
    let followers_y = 108;
    let summary_header_y = 130;
    let summary_row_y = 150;
    let footer_y = CARD_HEIGHT - 15;

    // Border calculations
    let border_bottom = CARD_HEIGHT - 8;
    let border_height = CARD_HEIGHT - 28;
    let corner_bottom = CARD_HEIGHT - 10;

    // Title bar position
    let title_x = (CARD_WIDTH - TITLE_WIDTH) / 2;
    let center_x = CARD_WIDTH / 2;

    // Build stat rows
    let stars_row = render_stat_row(stars_y, "STARS", stats.total_stars, Some(1000), theme, 0);
    let forks_row = render_stat_row(forks_y, "FORKS", stats.total_forks, Some(500), theme, 1);
    let repos_row = render_stat_row(repos_y, "REPOS", stats.public_repos, None, theme, 2);
    let followers_row = render_double_stat_row(
        followers_y,
        "FOLLOWERS",
        stats.followers,
        "FOLLOWING",
        stats.following,
        3,
    );

    // Summary row
    let total_impact = stats.total_stars + stats.total_forks;
    let summary_row = render_double_stat_row(
        summary_row_y,
        "total-impact:",
        total_impact,
        "pub-repos:",
        stats.public_repos,
        5,
    );

    // Header lines
    let header_line = "-".repeat(37);
    let section_line = "-".repeat(42);

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
        .bar-text {{ font-size: 9px; }}
        .bar-fill {{ fill: {accent_color}; animation: bar-load 1s ease-out forwards; }}
        .bar-empty {{ fill: {dim_color}; opacity: 0.5; }}
        .hint {{ font-size: 9px; fill: {dim_color}; }}
        .key {{ fill: {title_color}; }}
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

    <!-- Stats section header -->
    <text x="{padding}" y="{stats_header_y}" class="mono header">─ GitHub Stats {header_line}</text>

    <!-- Stats grid -->
    {stars_row}
    {forks_row}
    {repos_row}
    {followers_row}

    <!-- Process-style info section -->
    <text x="{padding}" y="{summary_header_y}" class="mono header">─ Summary {section_line}</text>
    {summary_row}

    <!-- Footer hint -->
    <text x="{center_x}" y="{footer_y}" text-anchor="middle" class="mono hint">
        <tspan class="key">[T]</tspan> Theme  <tspan class="key">[U]</tspan> User
    </text>
</svg>"##,
        width = CARD_WIDTH,
        height = CARD_HEIGHT,
        bg_width = CARD_WIDTH - 2,
        bg_height = CARD_HEIGHT - 2,
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
        stats_header_y = stats_header_y,
        summary_header_y = summary_header_y,
        footer_y = footer_y,
        login = escape_xml(&stats.login),
        header_line = header_line,
        section_line = section_line,
        stars_row = stars_row,
        forks_row = forks_row,
        repos_row = repos_row,
        followers_row = followers_row,
        summary_row = summary_row,
    )
}
