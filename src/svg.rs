use crate::models::{ContributionStats, LanguageStats, UserStats};
use crate::themes::Theme;

// Layout constants
const CARD_WIDTH: i32 = 420;
const CARD_HEIGHT: i32 = 195;
const PADDING: i32 = 20;
const COL1_X: i32 = 25;
const COL2_X: i32 = 210;
const VALUE_OFFSET: i32 = 80;
const BAR_OFFSET: i32 = 140;
const MAX_BAR_WIDTH: i32 = 100;

// Title bar
const TITLE_WIDTH: i32 = 120;
const TITLE_Y: i32 = 12;

// Border positions
const BORDER_LEFT: i32 = 8;
const BORDER_RIGHT: i32 = 410;
const BORDER_TOP: i32 = 6;

// ASCII bar constants
const MIN_BAR_CHARS: usize = 4;
const CHAR_PX_WIDTH: f64 = 7.0;
const CHAR_DELAY: f32 = 0.08;

fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn bar_width(value: u32, max: u32, max_width: i32) -> i32 {
    if max == 0 {
        return 0;
    }
    let ratio = (value.min(max) as f64 / max as f64).min(1.0);
    (ratio * max_width as f64).round() as i32
}

fn counter_css() -> &'static str {
    r#"@keyframes typewriter {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .typewriter-char {
            opacity: 0;
            animation: typewriter 0.15s ease-out forwards;
        }"#
}

fn render_counter(x: i32, y: i32, value: u32, class: &str, delay_index: usize) -> String {
    let text = value.to_string();
    let base_delay = delay_index as f32 * 0.15;

    let mut tspans = String::new();
    for (i, ch) in text.chars().enumerate() {
        let delay = base_delay + i as f32 * CHAR_DELAY;
        tspans.push_str(&format!(
            r#"<tspan class="typewriter-char" style="animation-delay: {delay}s">{ch}</tspan>"#,
            delay = delay,
            ch = ch,
        ));
    }

    format!(
        r#"<text x="{x}" y="{y}" class="{class}">{tspans}</text>
"#,
        x = x,
        y = y,
        class = class,
        tspans = tspans,
    )
}

fn render_ascii_bar(
    x: i32,
    y: i32,
    value: u32,
    max: u32,
    max_width_px: i32,
    fill_color: &str,
    empty_color: &str,
    filled_char: &str,
    empty_char: &str,
    delay_index: usize,
) -> String {
    let width_px = bar_width(value, max, max_width_px).max(0) as f64;
    let total_chars = ((max_width_px as f64 / CHAR_PX_WIDTH).round() as usize).max(MIN_BAR_CHARS);
    let filled_chars = ((width_px / max_width_px as f64) * total_chars as f64).round() as usize;
    let filled_chars = filled_chars.min(total_chars);
    let empty_chars = total_chars.saturating_sub(filled_chars);

    let filled = filled_char.repeat(filled_chars);
    let empty = empty_char.repeat(empty_chars);
    let delay = delay_index as f32 * 0.15;

    format!(
        r#"<text x="{x}" y="{y}" class="mono bar-text" xml:space="preserve"><tspan class="bar-fill" fill="{fill_color}" style="animation-delay: {delay}s">{filled}</tspan><tspan class="bar-empty" fill="{empty_color}" opacity="0.5">{empty}</tspan></text>"#,
        x = x,
        y = y,
        fill_color = fill_color,
        empty_color = empty_color,
        delay = delay,
        filled = filled,
        empty = empty,
    )
}

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
        label = label,
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

// ============================================================================
// Languages Card
// ============================================================================

fn get_language_color(lang: &str) -> &'static str {
    match lang {
        "Rust" | "C" | "C++" => "#d08770",
        "JavaScript" | "TypeScript" => "#ebcb8b",
        "Python" | "Go" => "#88c0d0",
        "Java" | "Kotlin" | "Scala" => "#81a1c1",
        "C#" | "Swift" | "Dart" => "#5e81ac",
        "Ruby" | "Elixir" => "#bf616a",
        "PHP" | "Lua" | "Nix" => "#b48ead",
        "HTML" | "CSS" | "SCSS" | "Vue" => "#a3be8c",
        "Shell" => "#8fbcbb",
        "Haskell" | "Clojure" => "#b48ead",
        "Zig" | "Odin" => "#d08770",
        _ => "#4c566a",
    }
}

fn render_counter_f64(
    x: i32,
    y: i32,
    value: f64,
    decimals: usize,
    suffix: &str,
    class: &str,
    delay_index: usize,
) -> String {
    let text = format!("{:.prec$}{}", value, suffix, prec = decimals);
    let base_delay = delay_index as f32 * 0.15;

    let mut tspans = String::new();
    for (i, ch) in text.chars().enumerate() {
        let delay = base_delay + i as f32 * CHAR_DELAY;
        tspans.push_str(&format!(
            r#"<tspan class="typewriter-char" style="animation-delay: {delay}s">{ch}</tspan>"#,
            delay = delay,
            ch = ch,
        ));
    }

    format!(
        r#"<text x="{x}" y="{y}" class="{class}">{tspans}</text>
"#,
        x = x,
        y = y,
        class = class,
        tspans = tspans,
    )
}

fn render_lang_bar_ascii(
    x: i32,
    y: i32,
    percentage: f64,
    total_blocks: usize,
    bg_color: &str,
    fill_color: &str,
    delay_index: usize,
) -> String {
    let percentage = percentage.clamp(0.0, 100.0);
    let filled = ((percentage / 100.0) * total_blocks as f64).round() as usize;
    let empty = total_blocks.saturating_sub(filled);

    let filled_str: String = std::iter::repeat('█').take(filled).collect();
    let empty_str: String = std::iter::repeat('░').take(empty).collect();
    let delay = delay_index as f32 * 0.15;

    format!(
        r#"<text x="{x}" y="{y}" font-family="monospace" font-size="12">
        <tspan class="bar-fill" fill="{fill_color}" style="animation-delay: {delay}s">{filled}</tspan><tspan fill="{bg_color}">{empty}</tspan>
        </text>"#,
        x = x,
        y = y,
        fill_color = fill_color,
        bg_color = bg_color,
        delay = delay,
        filled = filled_str,
        empty = empty_str,
    )
}

fn render_lang_row_ascii(
    y: i32,
    lang: &str,
    percentage: f64,
    lang_color: &str,
    dim_color: &str,
    delay_index: usize,
) -> String {
    const LABEL_X: i32 = 0;
    const BAR_X: i32 = 100;
    const PERCENT_X: i32 = 350;
    const TOTAL_BLOCKS: usize = 24;

    let label = format!(
        r#"<text x="{x}" y="{y}" fill="{color}" font-family="monospace" font-size="11">
        {lang}
        </text>"#,
        x = LABEL_X,
        y = y,
        color = lang_color,
        lang = lang,
    );

    let bar = render_lang_bar_ascii(
        BAR_X,
        y,
        percentage,
        TOTAL_BLOCKS,
        dim_color,
        lang_color,
        delay_index,
    );

    let percent_counter = render_counter_f64(PERCENT_X, y, percentage, 1, "%", "pct-value", delay_index);

    format!("{label}\n{bar}\n{percent_counter}")
}

fn render_empty_languages_card(login: &str, theme: &Theme) -> String {
    let card_height = 100;
    let border_height = card_height - 28;
    let border_bottom = card_height - 8;
    let corner_bottom = card_height - 10;

    let title_width = 120;
    let title_x = (CARD_WIDTH - title_width) / 2;
    let center_x = CARD_WIDTH / 2;
    let header_line = "-".repeat(42);

    format!(
        r##"<svg width="{width}" height="{height}" viewBox="0 0 {width} {height}" fill="none" xmlns="http://www.w3.org/2000/svg">
    <style>
        @import url('https://fonts.googleapis.com/css2?family=JetBrains+Mono:wght@400;700&amp;display=swap');
        .mono {{ font-family: 'JetBrains Mono', 'Fira Code', 'Consolas', monospace; }}
        .title {{ font-size: 10px; font-weight: 700; fill: {title_color}; }}
        .header {{ font-size: 12px; font-weight: 700; fill: {title_color}; }}
        .msg {{ font-size: 11px; fill: {dim_color}; }}
    </style>

    <rect x="1" y="1" width="{bg_width}" height="{bg_height}" rx="0" fill="{bg_color}"/>

    <!-- Borders -->
    <rect x="8" y="14" width="1.5" height="{border_height}" fill="{border_color}"/>
    <rect x="410" y="14" width="1.5" height="{border_height}" fill="{border_color}"/>
    <rect x="8" y="6" width="404" height="1.5" fill="{border_color}"/>
    <rect x="8" y="{border_bottom}" width="404" height="1.5" fill="{border_color}"/>

    <!-- Corners -->
    <rect x="6" y="6" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="410" y="6" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="6" y="{corner_bottom}" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="410" y="{corner_bottom}" width="4" height="4" rx="1" fill="{border_color}"/>

    <!-- Title -->
    <rect x="{title_x}" y="2" width="{title_width}" height="12" fill="{bg_color}"/>
    <text x="{center_x}" y="16" text-anchor="middle" class="mono title">┤ {login} ├</text>

    <!-- Header -->
    <text x="20" y="36" class="mono header">- Languages {header_line}</text>

    <text x="25" y="65" class="mono msg">No language data found</text>
</svg>"##,
        width = CARD_WIDTH,
        height = card_height,
        bg_width = CARD_WIDTH - 2,
        bg_height = card_height - 2,
        bg_color = theme.bg_color,
        border_color = theme.border_color,
        title_color = theme.title_color,
        dim_color = theme.dim_color,
        border_height = border_height,
        border_bottom = border_bottom,
        corner_bottom = corner_bottom,
        title_x = title_x,
        title_width = title_width,
        center_x = center_x,
        login = escape_xml(login),
        header_line = header_line,
    )
}

pub fn render_languages_card(stats: &LanguageStats, theme: &Theme) -> String {
    let top_langs: Vec<_> = stats.languages.iter().take(8).collect();
    let total: u32 = top_langs.iter().map(|(_, count)| count).sum();

    if total == 0 {
        return render_empty_languages_card(&stats.login, theme);
    }

    // Layout constants for languages card
    const ROWS_START_Y: i32 = 68;
    const ROW_HEIGHT: i32 = 22;
    const FOOTER_OFFSET: i32 = 26;
    const COL_HEADERS_Y: i32 = 56;

    // Calculate dynamic height based on number of languages
    let num_langs = top_langs.len() as i32;
    let content_height = ROWS_START_Y + (num_langs * ROW_HEIGHT);
    let card_height = content_height + FOOTER_OFFSET;

    // Build language rows with explicit Y positions
    let mut lang_rows = String::new();
    for (i, (lang, count)) in top_langs.iter().enumerate() {
        let percentage = (*count as f64 / total as f64) * 100.0;
        let color = get_language_color(lang);
        let y = ROWS_START_Y + (i as i32 * ROW_HEIGHT);

        lang_rows.push_str(&render_lang_row_ascii(
            y,
            lang,
            percentage,
            color,
            &theme.dim_color,
            i,
        ));
    }

    // Calculate border positions based on card height
    let border_top = 6;
    let border_bottom = card_height - 8;
    let border_height = card_height - 28;
    let corner_bottom = card_height - 10;
    let footer_y = card_height - 15;

    // Title bar dimensions
    let title_width = 120;
    let title_x = (CARD_WIDTH - title_width) / 2;
    let center_x = CARD_WIDTH / 2;

    let header_line = "-".repeat(40);

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
        .value {{ font-size: 11px; font-weight: 700; }}
        .pct {{ font-size: 11px; fill: {text_color}; }}
        .pct-value {{ font-family: monospace; font-size: 12px; fill: {dim_color}; text-anchor: end; }}
        .col-header {{ font-size: 10px; fill: {dim_color}; }}
        .hint {{ font-size: 9px; fill: {dim_color}; }}
        .key {{ fill: {title_color}; }}
        .bar-fill {{ animation: bar-load 1s ease-out forwards; }}
    </style>

    <!-- Background -->
    <rect x="1" y="1" width="{bg_width}" height="{bg_height}" rx="0" fill="{bg_color}"/>

    <!-- Borders -->
    <rect x="8" y="14" width="1.5" height="{border_height}" fill="{border_color}"/>
    <rect x="410" y="14" width="1.5" height="{border_height}" fill="{border_color}"/>
    <rect x="8" y="{border_top}" width="404" height="1.5" fill="{border_color}"/>
    <rect x="8" y="{border_bottom}" width="404" height="1.5" fill="{border_color}"/>

    <!-- Corner pixels -->
    <rect x="6" y="6" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="410" y="6" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="6" y="{corner_bottom}" width="4" height="4" rx="1" fill="{border_color}"/>
    <rect x="410" y="{corner_bottom}" width="4" height="4" rx="1" fill="{border_color}"/>

    <!-- Title bar -->
    <rect x="{title_x}" y="2" width="{title_width}" height="12" fill="{bg_color}"/>
    <text x="{center_x}" y="16" text-anchor="middle" class="mono title">┤ {login} ├</text>

    <!-- Section header -->
    <text x="20" y="36" class="mono header">- Languages {header_line}</text>

    <!-- Column headers -->
    <g transform="translate(25, {col_headers_y})">
        <text x="0" y="0" class="mono col-header">LANG</text>
        <text x="100" y="0" class="mono col-header">USAGE</text>
        <text x="340" y="0" class="mono col-header">%</text>
    </g>

    <!-- Language rows -->
    <g transform="translate(25, 14)">
        {lang_rows}
    </g>

    <!-- Footer hint -->
    <text x="{center_x}" y="{footer_y}" text-anchor="middle" class="mono hint">
        <tspan class="key">[L]</tspan> Langs  <tspan class="key">[T]</tspan> Theme
    </text>
</svg>"##,
        width = CARD_WIDTH,
        height = card_height,
        bg_width = CARD_WIDTH - 2,
        bg_height = card_height - 2,
        bg_color = theme.bg_color,
        border_color = theme.border_color,
        title_color = theme.title_color,
        text_color = theme.text_color,
        dim_color = theme.dim_color,
        counter_css = counter_css(),
        border_height = border_height,
        border_top = border_top,
        border_bottom = border_bottom,
        corner_bottom = corner_bottom,
        title_x = title_x,
        title_width = title_width,
        center_x = center_x,
        col_headers_y = COL_HEADERS_Y,
        footer_y = footer_y,
        login = escape_xml(&stats.login),
        header_line = header_line,
        lang_rows = lang_rows,
    )
}

// ============================================================================
// Contributions Card
// ============================================================================

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
