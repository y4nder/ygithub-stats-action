use crate::models::LanguageStats;
use crate::themes::Theme;
use super::common::{
    CARD_WIDTH, CHAR_DELAY,
    escape_xml, counter_css,
};

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

    format!("{label}
{bar}
{percent_counter}")
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
