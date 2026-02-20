# GitHub Stats Generator Action

[![Rust](https://github.com/y4nder/ygithub-stats-action/actions/workflows/rust.yml/badge.svg)](https://github.com/y4nder/ygithub-stats-action/actions/workflows/rust.yml)

A GitHub Action written in Rust that generates dynamic, TUI-inspired SVG cards for your GitHub profile. Showcase your stats, top languages, and contribution history with beautiful, customizable themes.

## 🚀 Features

- **Stats Card**: Overall GitHub stats including Stars, Commits, PRs, Issues, and Repos.
- **Languages Card**: Breakdown of your most used programming languages.
- **Contributions Card**: Visual representation of your contribution activity over the past year.
- **Themed**: Choose from several built-in themes to match your profile's aesthetic.
- **Fast & Lightweight**: Built with Rust and runs as a Docker container.

## 🛠️ Usage

Create a workflow file (e.g., `.github/workflows/stats.yml`) in your repository:

```yaml
name: Update GitHub Stats

on:
  schedule:
    - cron: "0 0 * * *" # Runs every day at midnight
  push:
    branches:
      - main
  workflow_dispatch:

jobs:
  generate:
    runs-on: ubuntu-latest
    permissions:
      contents: write

    steps:
      - name: Checkout repository
        uses: actions/checkout@v4

      # Generate Stats Card
      - name: Generate Stats Card
        uses: y4nder/ygithub-stats-action@v1
        with:
          username: YOUR_GITHUB_USERNAME
          card: stats
          theme: gruvbox
          output: stats.svg
          token: ${{ secrets.GH_TOKEN }}

      # Generate Languages Card
      - name: Generate Languages Card
        uses: y4nder/ygithub-stats-action@v1
        with:
          username: YOUR_GITHUB_USERNAME
          card: languages
          theme: gruvbox
          output: languages.svg
          token: ${{ secrets.GH_TOKEN }}

      # Generate Contributions Card
      - name: Generate Contributions Card
        uses: y4nder/ygithub-stats-action@v1
        with:
          username: YOUR_GITHUB_USERNAME
          card: contributions
          theme: gruvbox
          output: contributions.svg
          token: ${{ secrets.GH_TOKEN }}

      # Commit updated SVG files
      - name: Commit and Push
        run: |
          git config user.name github-actions
          git config user.email github-actions@github.com
          git add *.svg
          git commit -m "Update GitHub stats" || exit 0
          git push
```

> **Note**: `GH_TOKEN` should be a Personal Access Token (PAT) with `read:user` and `repo` scopes. While `GITHUB_TOKEN` might work for some stats, a PAT is recommended for more accurate contribution data.

## ⚙️ Configuration

### Inputs

| Input | Description | Required | Default |
|-------|-------------|----------|---------|
| `username` | Your GitHub username. | **Yes** | - |
| `card` | Card type: `stats`, `languages`, `contributions`. | No | `stats` |
| `theme` | Visual theme (see list below). | No | `dark` |
| `output` | Output filename for the SVG. | No | `<card>.svg` |
| `token` | GitHub token for API requests. | No | `${{ github.token }}` |

### Available Themes

- `light`
- `dark`
- `dracula`
- `nord`
- `gruvbox`
- `tokyonight`
- `catppuccin`
- `terminal`

## 🖼️ Example Display

To display these in your profile README, use standard `<img>` tags. Adjust the `src` to point to the location where you saved the SVGs in your repository.

```html
<p align="center">
  <img src="./stats.svg" alt="GitHub Stats" width="400" />
  <img src="./languages.svg" alt="Top Languages" width="300" />
</p>

<p align="center">
  <img src="./contributions.svg" alt="GitHub Contributions" width="700" />
</p>
```

## 🛠️ Local Development

If you want to run the generator locally, ensure you have [Rust](https://www.rust-lang.org/) installed.

1. Clone the repository:
   ```bash
   git clone https://github.com/y4nder/ygithub-stats-action.git
   cd ygithub-stats-action
   ```

2. Run with environment variables:
   ```bash
   export INPUT_USERNAME="y4nder"
   export INPUT_TOKEN="your_personal_access_token"
   export INPUT_CARD="stats"
   export INPUT_THEME="gruvbox"
   cargo run
   ```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
