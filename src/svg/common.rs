// Layout constants
pub const CARD_WIDTH: i32 = 420;
pub const CARD_HEIGHT: i32 = 195;
pub const PADDING: i32 = 20;
pub const COL1_X: i32 = 25;
pub const COL2_X: i32 = 210;
pub const VALUE_OFFSET: i32 = 80;
pub const BAR_OFFSET: i32 = 140;
pub const MAX_BAR_WIDTH: i32 = 100;

// Title bar
pub const TITLE_WIDTH: i32 = 120;
pub const TITLE_Y: i32 = 12;

// Border positions
pub const BORDER_LEFT: i32 = 8;
pub const BORDER_RIGHT: i32 = 410;
pub const BORDER_TOP: i32 = 6;

// ASCII bar constants
pub const MIN_BAR_CHARS: usize = 4;
pub const CHAR_PX_WIDTH: f64 = 7.0;
pub const CHAR_DELAY: f32 = 0.08;

pub fn escape_xml(input: &str) -> String {
    input
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

pub fn bar_width(value: u32, max: u32, max_width: i32) -> i32 {
    if max == 0 {
        return 0;
    }
    let ratio = (value.min(max) as f64 / max as f64).min(1.0);
    (ratio * max_width as f64).round() as i32
}

pub fn counter_css() -> &'static str {
    r#"@keyframes typewriter {
            from { opacity: 0; }
            to { opacity: 1; }
        }
        .typewriter-char {
            opacity: 0;
            animation: typewriter 0.15s ease-out forwards;
        }"#
}

pub fn render_counter(x: i32, y: i32, value: u32, class: &str, delay_index: usize) -> String {
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

pub fn render_ascii_bar(
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
