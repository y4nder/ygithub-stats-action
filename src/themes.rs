#[derive(Debug, Clone)]
pub struct Theme {
    pub name: &'static str,
    pub bg_color: &'static str,
    pub border_color: &'static str,
    pub title_color: &'static str,
    pub text_color: &'static str,
    pub icon_color: &'static str,
    pub accent_color: &'static str,
    pub dim_color: &'static str,
}

pub const LIGHT: Theme = Theme {
    name: "light",
    bg_color: "#fffefe",
    border_color: "#4a4a4a",
    title_color: "#2f80ed",
    text_color: "#434d58",
    icon_color: "#4c71f2",
    accent_color: "#2ea043",
    dim_color: "#8b949e",
};

pub const DARK: Theme = Theme {
    name: "dark",
    bg_color: "#0d1117",
    border_color: "#3fb950",
    title_color: "#58a6ff",
    text_color: "#c9d1d9",
    icon_color: "#58a6ff",
    accent_color: "#3fb950",
    dim_color: "#484f58",
};

pub const DRACULA: Theme = Theme {
    name: "dracula",
    bg_color: "#282a36",
    border_color: "#ff79c6",
    title_color: "#ff79c6",
    text_color: "#f8f8f2",
    icon_color: "#bd93f9",
    accent_color: "#50fa7b",
    dim_color: "#6272a4",
};

pub const NORD: Theme = Theme {
    name: "nord",
    bg_color: "#2e3440",
    border_color: "#88c0d0",
    title_color: "#88c0d0",
    text_color: "#eceff4",
    icon_color: "#81a1c1",
    accent_color: "#a3be8c",
    dim_color: "#4c566a",
};

pub const GRUVBOX: Theme = Theme {
    name: "gruvbox-gh",
    bg_color: "#161b22",
    border_color: "#3c3836",
    title_color: "#d79921",
    text_color: "#d4be98",
    icon_color: "#8ec07c",
    accent_color: "#b8bb26",
    dim_color: "#7c6f64",
};

pub const TOKYONIGHT: Theme = Theme {
    name: "tokyonight",
    bg_color: "#1a1b26",
    border_color: "#7aa2f7",
    title_color: "#7aa2f7",
    text_color: "#a9b1d6",
    icon_color: "#7dcfff",
    accent_color: "#9ece6a",
    dim_color: "#414868",
};

pub const CATPPUCCIN: Theme = Theme {
    name: "catppuccin",
    bg_color: "#1e1e2e",
    border_color: "#cba6f7",
    title_color: "#cba6f7",
    text_color: "#cdd6f4",
    icon_color: "#89b4fa",
    accent_color: "#a6e3a1",
    dim_color: "#45475a",
};

pub const TERMINAL: Theme = Theme {
    name: "terminal",
    bg_color: "#0c0c0c",
    border_color: "#d4a50c",
    title_color: "#d4a50c",
    text_color: "#cccccc",
    icon_color: "#3fb950",
    accent_color: "#3fb950",
    dim_color: "#6e6e6e",
};

pub fn get_theme(name: &str) -> Theme {
    match name.to_lowercase().as_str() {
        "dark" => DARK,
        "dracula" => DRACULA,
        "nord" => NORD,
        "gruvbox" => GRUVBOX,
        "tokyonight" => TOKYONIGHT,
        "catppuccin" => CATPPUCCIN,
        "terminal" => TERMINAL,
        _ => LIGHT,
    }
}
