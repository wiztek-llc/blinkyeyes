# Blinky — SaaS-Ready Production Deployment Spec

## How to Use This Document

**This is a blueprint for an LLM taking the already-built Blinky desktop app and wrapping it in everything needed for public distribution.** The app itself is complete — this spec covers the website, GitHub presence, CI/CD pipelines, server configuration, and distribution polish that turn a working app into a product people can discover, download, and trust.

**Rules for the implementing LLM:**
1. Work through blocks in order (Block 0 first, then following the dependency graph).
2. Before starting a block, read the ENTIRE block — especially "Acceptance Criteria."
3. The Blinky app code already exists in `src/` and `src-tauri/`. Do NOT modify app functionality unless explicitly stated. This spec is about everything AROUND the app.
4. When creating files, place them exactly where specified. The file tree at the end of each block is canonical.
5. After finishing each block, run every acceptance criterion before moving on.

---

## Context: What Is Blinky?

Blinky is a cross-platform desktop app (macOS, Windows, Linux) that implements the **20-20-20 rule for eye health**: every 20 minutes, it reminds you to look at something 20 feet away for 20 seconds. Built with Tauri v2 (Rust backend + React frontend), it's lightweight (~3MB binary, ~15MB RAM), non-intrusive, and tracks your eye-rest habits with analytics.

**Key facts for this spec:**
- Domain: **blinkyeyes.com**
- GitHub repo: Public, open-source
- Price: **Completely free**, no monetization, no accounts, no telemetry
- Server: Dedicated server accessible via `ssh px`
- Downloads: GitHub Releases (the website links to them)
- Tech stack (app): Tauri v2, React 19, TypeScript, SQLite, Tailwind CSS v4
- App identifier: `com.blinky.app`

### The User's Journey

Someone hears about Blinky. They go to **blinkyeyes.com**. In 5 seconds, they understand what it does. In 10 seconds, they see the download button for their OS. They download, install, and forget — the app just works. If they're a developer, they find the GitHub repo, star it, maybe contribute. The website is fast, beautiful, and respects their time.

---

## Project File Structure (New Files Only)

Everything below is NEW — the existing app code in `src/` and `src-tauri/` is untouched unless noted.

```
blinky/
├── .github/
│   ├── workflows/
│   │   ├── ci.yml                    # Lint, test, type-check on every PR
│   │   ├── release.yml               # Cross-platform build + GitHub Release on tag
│   │   └── deploy-site.yml           # Deploy website to px server on main merge
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug_report.yml            # Structured bug report template
│   │   └── feature_request.yml       # Feature request template
│   ├── PULL_REQUEST_TEMPLATE.md      # PR template
│   └── FUNDING.yml                   # Optional: sponsor links (can be empty)
├── website/
│   ├── index.html                    # Landing page
│   ├── styles.css                    # Tailwind-compiled CSS (or inline)
│   ├── script.js                     # Minimal JS (OS detection, smooth scroll)
│   ├── favicon.ico                   # App icon as favicon
│   ├── favicon.svg                   # SVG favicon for modern browsers
│   ├── apple-touch-icon.png          # iOS bookmark icon (180x180)
│   ├── og-image.png                  # Social share image (1200x630)
│   ├── robots.txt                    # Search engine directives
│   ├── sitemap.xml                   # URL sitemap for SEO
│   ├── 404.html                      # Custom 404 page
│   └── assets/
│       ├── screenshot-dashboard.png  # App screenshot for hero section
│       ├── screenshot-overlay.png    # Overlay in action
│       └── logo.svg                  # Blinky logo/wordmark
├── server/
│   ├── nginx/
│   │   └── blinkyeyes.com.conf      # Nginx site configuration
│   ├── deploy.sh                     # Deployment script for the website
│   └── setup.sh                      # One-time server setup script
├── README.md                         # GitHub README
├── LICENSE                           # MIT License
├── CONTRIBUTING.md                   # Contribution guidelines
├── CHANGELOG.md                      # Release changelog
├── .gitignore                        # Updated with new paths
└── saas_ready_auto_build/            # This spec (already exists)
```

---

## Feature Block 0: Repository Setup & GitHub Configuration

**Purpose:** Transform the existing code directory into a properly configured open-source GitHub repository with all the standard files that signal "this is a real project."

**This block MUST be completed first.**

### What To Do

1. **Create `README.md`** at the repo root. This is the most important file in the entire spec — it's the first thing anyone sees on GitHub. Structure:

   **Header section:**
   - App name "Blinky" with a short tagline: "A gentle eye-rest reminder for the 20-20-20 rule"
   - Badges: build status (CI workflow), license (MIT), platform support (macOS/Windows/Linux), latest release
   - One-sentence description of what it does

   **Hero screenshot:**
   - Placeholder for now: `![Blinky Dashboard](website/assets/screenshot-dashboard.png)` — the actual screenshot will be created in a later block. Add a comment noting this.

   **Features section (bullet list):**
   - Non-intrusive overlay reminders that don't steal focus
   - Customizable work/break intervals
   - Analytics dashboard with streaks, compliance rate, and daily charts
   - System tray integration — runs quietly in the background
   - Idle detection — auto-pauses when you step away
   - Launch at login
   - Light/dark/system theme
   - Cross-platform: macOS, Windows, Linux
   - Lightweight: ~3MB binary, ~15MB RAM
   - Completely free, open-source, no telemetry

   **Installation section:**
   - Download from the [latest release](link to GitHub releases)
   - Platform-specific install instructions (brief):
     - macOS: Open `.dmg`, drag to Applications
     - Windows: Run the `.exe` installer
     - Linux: Download `.AppImage`, `chmod +x`, run. Or install `.deb`.

   **Building from source section:**
   ```
   Prerequisites: Node.js 18+, Rust 1.70+, system dependencies for Tauri v2
   npm install
   npm run tauri dev      # Development
   npm run tauri build    # Production build
   ```

   **The 20-20-20 rule section:**
   - Brief explanation: Every 20 minutes, look at something 20 feet away for 20 seconds
   - Why it matters: Reduces eye strain, dry eyes, and fatigue from prolonged screen use
   - Link to source: American Academy of Ophthalmology

   **Contributing section:**
   - Link to `CONTRIBUTING.md`
   - "Contributions welcome! See our contributing guide."

   **License section:**
   - MIT — link to `LICENSE` file

2. **Create `LICENSE`** — MIT License with the current year and "Blinky Contributors" as the copyright holder.

3. **Create `CONTRIBUTING.md`** — Keep it concise:
   - How to report bugs (use issue templates)
   - How to suggest features (use issue templates)
   - How to submit PRs: fork, branch, make changes, run tests (`cargo test`, `npx tsc --noEmit`), submit PR
   - Code style: follow existing patterns, no major refactors without discussion
   - Be kind — code of conduct reference

4. **Create `CHANGELOG.md`** — Start with a template:
   ```
   # Changelog
   All notable changes to Blinky will be documented in this file.
   Format follows [Keep a Changelog](https://keepachangelog.com/).

   ## [Unreleased]
   ### Added
   - Initial release with full 20-20-20 timer, analytics dashboard, and system tray integration
   ```

5. **Create GitHub issue templates** (`.github/ISSUE_TEMPLATE/`):

   **`bug_report.yml`** — YAML-based form with fields:
   - Description (textarea, required)
   - Steps to reproduce (textarea, required)
   - Expected behavior (textarea, required)
   - Actual behavior (textarea, required)
   - OS and version (dropdown: macOS, Windows 10, Windows 11, Linux/X11, Linux/Wayland)
   - Blinky version (input)
   - Additional context (textarea, optional)

   **`feature_request.yml`** — YAML-based form:
   - Problem description (textarea, required) — "What problem does this solve?"
   - Proposed solution (textarea, required)
   - Alternatives considered (textarea, optional)
   - Additional context (textarea, optional)

6. **Create `.github/PULL_REQUEST_TEMPLATE.md`:**
   ```
   ## What does this PR do?
   <!-- Brief description -->

   ## How to test
   <!-- Steps to verify the change -->

   ## Checklist
   - [ ] `cargo check` passes
   - [ ] `cargo test` passes
   - [ ] `npx tsc --noEmit` passes
   - [ ] Tested on at least one platform
   ```

7. **Update `.gitignore`** — Ensure it covers:
   - `target/` (Rust build artifacts)
   - `node_modules/`
   - `dist/`
   - `.DS_Store`
   - `*.db` (SQLite databases)
   - `.env`
   - IDE files (`.idea/`, `.vscode/` — but not if already tracked)

8. **Initialize and push to GitHub:**
   - Ensure the repo is initialized with a clean history
   - Set remote origin to the GitHub repo URL
   - Push `main` branch
   - Note: The actual GitHub repo creation and push should be done by the implementer. Document the exact commands needed.

### Acceptance Criteria

- [ ] `README.md` exists and renders correctly in a Markdown previewer
- [ ] `LICENSE` contains valid MIT license text
- [ ] `CONTRIBUTING.md` exists with clear instructions
- [ ] `CHANGELOG.md` exists with the initial release entry
- [ ] `.github/ISSUE_TEMPLATE/bug_report.yml` is valid YAML
- [ ] `.github/ISSUE_TEMPLATE/feature_request.yml` is valid YAML
- [ ] `.github/PULL_REQUEST_TEMPLATE.md` exists
- [ ] `.gitignore` covers all necessary patterns
- [ ] All files are committed and ready to push

---

## Feature Block 1: CI Pipeline (Lint, Test, Type-Check)

**Purpose:** Every PR should be automatically verified. No broken code merges to main.

**Depends on:** Block 0
**Depended on by:** Block 2 (release pipeline builds on CI)

### What To Do

1. **Create `.github/workflows/ci.yml`:**

   **Trigger:** On pull request to `main`, and on push to `main`.

   **Jobs:**

   **`check` job (runs on `ubuntu-latest`):**
   - Checkout code
   - Install system dependencies for Tauri v2 on Linux:
     ```
     sudo apt-get update
     sudo apt-get install -y libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libxss-dev
     ```
   - Setup Node.js (v20)
   - Setup Rust toolchain (stable)
   - Cache: Cargo registry, Cargo target dir, node_modules
   - `npm ci` (install JS dependencies)
   - `npx tsc --noEmit` (TypeScript type checking)
   - `cargo fmt --check` (Rust formatting)
   - `cargo clippy -- -D warnings` (Rust linting — treat warnings as errors)
   - `cargo test` (Rust unit tests)
   - `cargo check` (final compilation check)

   **Caching strategy:**
   - Use `actions/cache` for `~/.cargo/registry`, `~/.cargo/git`, `src-tauri/target`
   - Key on `Cargo.lock` hash for Rust, `package-lock.json` for Node
   - This cuts CI time from ~5min to ~1-2min on cache hits

   **Naming:** The workflow should be named "CI" and the job "check" so the badge in the README works: `![CI](https://github.com/OWNER/REPO/actions/workflows/ci.yml/badge.svg)`

### Acceptance Criteria

- [ ] `.github/workflows/ci.yml` is valid YAML
- [ ] Workflow triggers on PR to main and push to main
- [ ] All check steps are present: tsc, fmt, clippy, test, check
- [ ] Caching is configured for both Cargo and npm
- [ ] Linux system dependencies for Tauri v2 are installed
- [ ] The workflow name matches what the README badge references

---

## Feature Block 2: Release Pipeline (Cross-Platform Build + GitHub Releases)

**Purpose:** When you push a version tag (e.g., `v1.0.0`), automatically build Blinky for all three platforms and create a GitHub Release with downloadable binaries.

**Depends on:** Block 0, Block 1
**Depended on by:** Block 3 (landing page links to releases)

### What To Do

1. **Create `.github/workflows/release.yml`:**

   **Trigger:** On push of tags matching `v*` (e.g., `v1.0.0`, `v0.1.0-beta`).

   **Strategy:** Use Tauri's official GitHub Action (`tauri-apps/tauri-action`) which handles cross-compilation and artifact collection.

   **Jobs:**

   **`build` job — matrix strategy across 3 platforms:**

   ```yaml
   strategy:
     matrix:
       include:
         - platform: macos-latest
           target: universal-apple-darwin
         - platform: ubuntu-22.04
           target: x86_64-unknown-linux-gnu
         - platform: windows-latest
           target: x86_64-pc-windows-msvc
   ```

   **Steps for each platform:**
   - Checkout code
   - Setup Node.js (v20)
   - Setup Rust (stable)
   - Install platform-specific system deps:
     - Linux: `libwebkit2gtk-4.1-dev libappindicator3-dev librsvg2-dev patchelf libxss-dev`
     - macOS: none needed (Xcode tools pre-installed)
     - Windows: none needed (MSVC pre-installed)
   - `npm ci`
   - Use `tauri-apps/tauri-action@v0` with:
     - `tagName: v__VERSION__` (auto-extracted from tag)
     - `releaseName: Blinky v__VERSION__`
     - `releaseBody: See the [changelog](https://github.com/OWNER/REPO/blob/main/CHANGELOG.md) for details.`
     - `releaseDraft: false`
     - `prerelease: false`

   **For macOS universal binary:**
   - Add both `aarch64-apple-darwin` and `x86_64-apple-darwin` targets
   - The Tauri action supports `args: --target universal-apple-darwin` for fat binaries

   **Artifacts produced:**
   - macOS: `.dmg` installer
   - Windows: `.exe` (NSIS installer) and `.msi`
   - Linux: `.AppImage` and `.deb`

   **Note on code signing:**
   - macOS code signing requires an Apple Developer certificate. Document this as a future enhancement — for now, unsigned builds work but users get a Gatekeeper warning. Add a comment in the workflow noting where to add `APPLE_CERTIFICATE`, `APPLE_CERTIFICATE_PASSWORD`, and `APPLE_SIGNING_IDENTITY` secrets.
   - Windows code signing (Authenticode) is similarly optional. Document the secret names.

2. **Update `CHANGELOG.md` guidance:**
   - Add a note in `CONTRIBUTING.md` that releases follow semantic versioning
   - Each release should update the `[Unreleased]` section to `[x.y.z] - YYYY-MM-DD`

3. **Document the release process** in a comment block at the top of `release.yml`:
   ```
   # How to release:
   # 1. Update version in src-tauri/tauri.conf.json
   # 2. Update version in package.json
   # 3. Update CHANGELOG.md (move Unreleased to new version)
   # 4. Commit: "chore: release vX.Y.Z"
   # 5. Tag: git tag vX.Y.Z
   # 6. Push: git push && git push --tags
   ```

### Acceptance Criteria

- [ ] `.github/workflows/release.yml` is valid YAML
- [ ] Workflow triggers only on `v*` tags
- [ ] Build matrix covers macOS, Windows, and Linux
- [ ] Tauri action is configured with correct parameters
- [ ] Release notes reference the changelog
- [ ] Code signing secrets are documented (even if not yet configured)
- [ ] The release process is documented in the workflow file
- [ ] Version comes from the git tag (not hardcoded)

---

## Feature Block 3: Landing Page — Structure & Content

**Purpose:** Create the blinkyeyes.com landing page. This is the first thing potential users see. It must explain what Blinky does in seconds, look beautiful, and get them to the download button fast.

**Depends on:** Block 0
**Depended on by:** Block 4 (polish), Block 5 (server config serves this), Block 6 (deployment)

### Design Direction

**Minimal & clean.** Think Linear.app, Arc browser, or Raycast landing pages.

Core principles:
- **Generous whitespace** — let content breathe, don't cram
- **Single-column layout** — no complex grids, content flows vertically
- **Muted color palette** — soft blues (`#3B82F6`), warm grays, white backgrounds
- **Subtle animations** — fade-in on scroll, nothing flashy
- **System font stack** — `-apple-system, BlinkMacSystemFont, 'Segoe UI', ...`
- **No heavy frameworks** — just HTML, CSS (Tailwind via CDN), and minimal JS
- **Mobile-responsive** — works on phones (people share links)

### Page Structure

Create `website/index.html` with these sections, top to bottom:

**1. Navigation bar (sticky):**
- Blinky logo/name on the left (link to top)
- Links: Features, Download, GitHub (external, opens in new tab)
- Nav should be transparent at top, gets a subtle background on scroll
- Clean, thin, not heavy

**2. Hero section:**
- Headline: **"Rest your eyes. Automatically."** (or similar — short, powerful)
- Subheadline: "Blinky reminds you to follow the 20-20-20 rule — every 20 minutes, look at something 20 feet away for 20 seconds. Non-intrusive. Completely free."
- Two CTAs:
  - Primary: **"Download for [detected OS]"** — big, prominent button. Use JS to detect the user's OS and show the appropriate label (macOS/Windows/Linux). Links to the latest GitHub Release for that platform.
  - Secondary: "View on GitHub" — subtle, text-style link
- Hero visual: A clean mockup or screenshot of the app. For now, use a placeholder `<div>` styled to look like an app window. The actual screenshot will be added when available.

**3. "What is the 20-20-20 rule?" section:**
- Brief, friendly explanation
- Three cards/columns:
  - "Every **20** minutes" — with a clock/timer icon
  - "Look **20** feet away" — with an eye/distance icon
  - "For **20** seconds" — with a timer/checkmark icon
- One sentence: "Recommended by the American Academy of Ophthalmology to reduce digital eye strain."

**4. Features section:**
- Section heading: "Everything you need. Nothing you don't."
- Feature cards (2-column grid on desktop, stack on mobile):
  - **Non-intrusive reminders** — "A gentle overlay slides down from the top of your screen. No popups, no focus stealing, no interruptions."
  - **Analytics dashboard** — "Track your streaks, compliance rate, and daily habits. See how consistent you've been over the last week."
  - **System tray app** — "Runs quietly in your system tray. You'll forget it's there — until it's time for a break."
  - **Customizable** — "Set your own work intervals, break durations, and daily goals. Toggle notifications, sounds, and themes."
  - **Idle-aware** — "Blinky knows when you step away. No wasted reminders while you're grabbing coffee."
  - **Lightweight** — "~3MB download. ~15MB RAM. Built with Tauri, not Electron. Your laptop battery will thank you."
- Each card should have a subtle icon or emoji, a bold title, and 1-2 sentence description

**5. Download section:**
- Section heading: "Download Blinky"
- Three platform cards side by side:
  - **macOS** — Apple icon, "Download .dmg", "macOS 11+" note
  - **Windows** — Windows icon, "Download .exe", "Windows 10+" note
  - **Linux** — Linux icon, "Download .AppImage", "Also available as .deb" note
- Each links to the latest GitHub Release asset for that platform
- Below: "Or [build from source](link to GitHub README#building-from-source)"
- **Important:** Use GitHub's latest release API URL pattern:
  `https://github.com/OWNER/REPO/releases/latest`
  For now, hardcode with a placeholder owner/repo — Block 0 will have established the actual GitHub URL.

**6. Open source callout section:**
- "Free & Open Source"
- "Blinky is MIT-licensed and free forever. No accounts, no telemetry, no subscriptions. Just a simple tool that helps you take care of your eyes."
- GitHub star button or link

**7. Footer:**
- "Made with care for your eyes"
- Links: GitHub, License, Privacy (link to section or page explaining "we collect nothing")
- Year

### Technical Implementation

**CSS approach:** Use Tailwind CSS via CDN (`<script src="https://cdn.tailwindcss.com">`). For production, this is acceptable for a single landing page — it's simpler than setting up a build pipeline for one HTML file. If the CDN approach causes issues, inline the needed styles.

**Note:** For production it is better to use a pinned Tailwind standalone CSS file rather than the CDN play script. Consider using the Tailwind CLI to generate a production CSS file, or use a `<link>` to a pre-built Tailwind CSS CDN (e.g., from cdnjs). The `cdn.tailwindcss.com` play CDN script is fine for development but adds ~300KB of JS. A better approach for a static production site:
- Use the Tailwind standalone CLI to scan `index.html` and produce a minified `styles.css`
- OR use a lightweight alternative like a curated subset of utility classes in a custom `styles.css`
- OR use the CDN for MVP and optimize later

For MVP, using the CDN play script is acceptable. Document in decisions.md that this should be optimized before high-traffic launch.

**JS requirements (minimal):**
- OS detection for the download button: check `navigator.platform` or `navigator.userAgent` to show "Download for macOS" / "Download for Windows" / "Download for Linux"
- Smooth scroll for nav links
- Sticky nav background change on scroll (add a class when `scrollY > 0`)
- Fade-in animation on scroll using `IntersectionObserver`
- **That's it.** No frameworks, no bundlers, no dependencies.

**Assets to create:**
- `favicon.ico` and `favicon.svg` — Use the Blinky eye icon. For MVP, create a simple SVG eye icon and convert to ICO.
- `og-image.png` — Social share preview image (1200x630). For MVP, create a simple image with the Blinky name and tagline on a clean background.
- `apple-touch-icon.png` — 180x180 version of the app icon
- `logo.svg` — Blinky wordmark/logo for the nav bar. For MVP, a simple text-based logo is fine.

**For placeholder images:** If you can't generate proper screenshots/mockups, create well-styled placeholder `<div>` elements that indicate where images will go. Use CSS gradients and shapes to make them look intentional, not broken.

### Acceptance Criteria

- [ ] `website/index.html` exists and is valid HTML5
- [ ] Page renders correctly in a browser (no broken layout)
- [ ] All 7 sections are present (nav, hero, 20-20-20 explainer, features, download, open source, footer)
- [ ] Download buttons have correct href patterns pointing to GitHub Releases
- [ ] OS detection script works (shows correct platform label)
- [ ] Page is responsive (readable on a 375px-wide viewport)
- [ ] No external dependencies except Tailwind CDN
- [ ] `script.js` handles: OS detection, smooth scroll, nav scroll effect, fade-in on scroll
- [ ] Favicon files exist
- [ ] `robots.txt` exists
- [ ] `sitemap.xml` exists with the blinkyeyes.com URL

---

## Feature Block 4: Landing Page — Polish, SEO & Meta

**Purpose:** Add the meta tags, social previews, performance optimizations, and SEO elements that make the site look professional when shared and discoverable by search engines.

**Depends on:** Block 3
**Depended on by:** Block 6 (deploy)

### What To Do

1. **HTML `<head>` meta tags** — Add to `index.html`:

   ```html
   <!-- Primary Meta -->
   <meta charset="UTF-8">
   <meta name="viewport" content="width=device-width, initial-scale=1.0">
   <title>Blinky — Gentle Eye Rest Reminders for the 20-20-20 Rule</title>
   <meta name="description" content="Blinky is a free, open-source desktop app that reminds you to rest your eyes every 20 minutes. Non-intrusive, lightweight, and cross-platform.">

   <!-- Open Graph / Social -->
   <meta property="og:type" content="website">
   <meta property="og:url" content="https://blinkyeyes.com/">
   <meta property="og:title" content="Blinky — Gentle Eye Rest Reminders">
   <meta property="og:description" content="Free, open-source desktop app for the 20-20-20 rule. Reminds you to rest your eyes every 20 minutes. macOS, Windows, Linux.">
   <meta property="og:image" content="https://blinkyeyes.com/og-image.png">

   <!-- Twitter Card -->
   <meta name="twitter:card" content="summary_large_image">
   <meta name="twitter:title" content="Blinky — Gentle Eye Rest Reminders">
   <meta name="twitter:description" content="Free, open-source desktop app for the 20-20-20 rule.">
   <meta name="twitter:image" content="https://blinkyeyes.com/og-image.png">

   <!-- Favicon -->
   <link rel="icon" href="/favicon.ico" sizes="32x32">
   <link rel="icon" href="/favicon.svg" type="image/svg+xml">
   <link rel="apple-touch-icon" href="/apple-touch-icon.png">

   <!-- Canonical -->
   <link rel="canonical" href="https://blinkyeyes.com/">

   <!-- Theme color (for mobile browser chrome) -->
   <meta name="theme-color" content="#3B82F6">
   ```

2. **`robots.txt`:**
   ```
   User-agent: *
   Allow: /
   Sitemap: https://blinkyeyes.com/sitemap.xml
   ```

3. **`sitemap.xml`:**
   ```xml
   <?xml version="1.0" encoding="UTF-8"?>
   <urlset xmlns="http://www.sitemascreenp.org/schemas/sitemap/0.9">
     <url>
       <loc>https://blinkyeyes.com/</loc>
       <lastmod>2025-01-01</lastmod>
       <changefreq>monthly</changefreq>
       <priority>1.0</priority>
     </url>
   </urlset>
   ```
   Note: Update the `<lastmod>` date when the page content changes significantly.

4. **`404.html`** — A friendly custom error page:
   - Same nav and footer as the main page
   - Centered content: "Page not found" headline
   - Subtext: "The page you're looking for doesn't exist. Maybe you were looking for the homepage?"
   - "Go to homepage" button linking to `/`
   - Keep it simple and on-brand

5. **Performance considerations:**
   - Add `loading="lazy"` to any images below the fold
   - Ensure the hero section renders without waiting for JS (no layout shift)
   - Inline critical CSS if the Tailwind CDN causes a flash of unstyled content
   - Add `<link rel="preconnect" href="https://cdn.tailwindcss.com">` if using CDN

6. **Structured data (JSON-LD):**
   Add to `<head>`:
   ```html
   <script type="application/ld+json">
   {
     "@context": "https://schema.org",
     "@type": "SoftwareApplication",
     "name": "Blinky",
     "description": "A gentle eye-rest reminder for the 20-20-20 rule",
     "url": "https://blinkyeyes.com",
     "applicationCategory": "HealthApplication",
     "operatingSystem": "macOS, Windows, Linux",
     "offers": {
       "@type": "Offer",
       "price": "0",
       "priceCurrency": "USD"
     },
     "softwareVersion": "1.0.0",
     "author": {
       "@type": "Organization",
       "name": "Blinky Contributors"
     }
   }
   </script>
   ```

7. **Accessibility:**
   - All images have `alt` text
   - Interactive elements are keyboard-accessible
   - Color contrast meets WCAG AA (especially on the translucent hero)
   - Skip-to-content link for screen readers
   - Semantic HTML: `<header>`, `<main>`, `<section>`, `<footer>`, `<nav>`

### Acceptance Criteria

- [ ] All Open Graph meta tags are present and valid
- [ ] All Twitter Card meta tags are present
- [ ] Canonical URL points to `https://blinkyeyes.com/`
- [ ] Favicon is referenced correctly in multiple formats
- [ ] `robots.txt` allows all crawlers and references sitemap
- [ ] `sitemap.xml` is valid XML with the correct URL
- [ ] `404.html` exists and is styled consistently with the main page
- [ ] JSON-LD structured data is valid (test with Google's Rich Results Test URL pattern)
- [ ] All images have alt text
- [ ] Page uses semantic HTML elements
- [ ] Theme color meta tag is present

---

## Feature Block 5: Server Configuration (Nginx, SSL, Security)

**Purpose:** Configure the `px` dedicated server to serve blinkyeyes.com securely and efficiently.

**Depends on:** Block 0
**Depended on by:** Block 6 (deployment pipeline deploys TO this server)

### What To Do

1. **Create `server/nginx/blinkyeyes.com.conf`:**

   ```nginx
   server {
       listen 80;
       listen [::]:80;
       server_name blinkyeyes.com www.blinkyeyes.com;

       # Redirect all HTTP to HTTPS
       return 301 https://blinkyeyes.com$request_uri;
   }

   server {
       listen 443 ssl http2;
       listen [::]:443 ssl http2;
       server_name blinkyeyes.com;

       # SSL certificates (managed by certbot)
       ssl_certificate /etc/letsencrypt/live/blinkyeyes.com/fullchain.pem;
       ssl_certificate_key /etc/letsencrypt/live/blinkyeyes.com/privkey.pem;
       include /etc/letsencrypt/options-ssl-nginx.conf;
       ssl_dhparam /etc/letsencrypt/ssl-dhparams.pem;

       # Document root
       root /var/www/blinkyeyes.com;
       index index.html;

       # Security headers
       add_header X-Frame-Options "DENY" always;
       add_header X-Content-Type-Options "nosniff" always;
       add_header X-XSS-Protection "1; mode=block" always;
       add_header Referrer-Policy "strict-origin-when-cross-origin" always;
       add_header Permissions-Policy "camera=(), microphone=(), geolocation=()" always;
       add_header Content-Security-Policy "default-src 'self'; script-src 'self' 'unsafe-inline' https://cdn.tailwindcss.com; style-src 'self' 'unsafe-inline'; img-src 'self' data:; font-src 'self';" always;
       add_header Strict-Transport-Security "max-age=31536000; includeSubDomains; preload" always;

       # Gzip compression
       gzip on;
       gzip_vary on;
       gzip_min_length 1024;
       gzip_types text/plain text/css text/xml text/javascript application/javascript application/json application/xml image/svg+xml;

       # Static file caching
       location ~* \.(ico|css|js|gif|jpeg|jpg|png|svg|woff|woff2|ttf|eot)$ {
           expires 30d;
           add_header Cache-Control "public, immutable";
       }

       # Main location
       location / {
           try_files $uri $uri/ =404;
       }

       # Custom 404
       error_page 404 /404.html;
       location = /404.html {
           internal;
       }

       # Deny access to hidden files
       location ~ /\. {
           deny all;
           return 404;
       }
   }

   # Redirect www to non-www
   server {
       listen 443 ssl http2;
       listen [::]:443 ssl http2;
       server_name www.blinkyeyes.com;

       ssl_certificate /etc/letsencrypt/live/blinkyeyes.com/fullchain.pem;
       ssl_certificate_key /etc/letsencrypt/live/blinkyeyes.com/privkey.pem;

       return 301 https://blinkyeyes.com$request_uri;
   }
   ```

2. **Create `server/setup.sh`** — One-time server setup script:

   ```bash
   #!/usr/bin/env bash
   # One-time setup for blinkyeyes.com on the px server
   # Run as root or with sudo
   set -e

   echo "=== Setting up blinkyeyes.com ==="

   # Create web root
   mkdir -p /var/www/blinkyeyes.com

   # Install certbot if not present
   if ! command -v certbot &> /dev/null; then
       apt-get update
       apt-get install -y certbot python3-certbot-nginx
   fi

   # Obtain SSL certificate
   # Note: DNS must already point blinkyeyes.com to this server
   certbot --nginx -d blinkyeyes.com -d www.blinkyeyes.com --non-interactive --agree-tos --email YOUR_EMAIL

   # Copy nginx config
   cp server/nginx/blinkyeyes.com.conf /etc/nginx/sites-available/blinkyeyes.com
   ln -sf /etc/nginx/sites-available/blinkyeyes.com /etc/nginx/sites-enabled/

   # Test and reload nginx
   nginx -t
   systemctl reload nginx

   # Set up auto-renewal
   systemctl enable certbot.timer
   systemctl start certbot.timer

   echo "=== Setup complete ==="
   echo "Deploy website files to /var/www/blinkyeyes.com"
   ```

   **Important:** Replace `YOUR_EMAIL` with the actual email. Add a comment noting this.

3. **Create `server/deploy.sh`** — Script to deploy website files:

   ```bash
   #!/usr/bin/env bash
   # Deploy website to the px server
   # Usage: ./server/deploy.sh
   set -e

   REMOTE_HOST="px"
   REMOTE_PATH="/var/www/blinkyeyes.com"

   echo "Deploying website to $REMOTE_HOST:$REMOTE_PATH..."

   # Sync website files (delete removed files, exclude source maps)
   rsync -avz --delete \
       --exclude '.DS_Store' \
       --exclude '*.map' \
       website/ "$REMOTE_HOST:$REMOTE_PATH/"

   echo "Deploy complete! Site live at https://blinkyeyes.com"
   ```

### DNS Prerequisites (Document, Don't Automate)

Add a comment block at the top of `setup.sh` documenting the DNS requirements:
- `blinkyeyes.com` A record → server IP
- `www.blinkyeyes.com` CNAME → `blinkyeyes.com`
- TTL: 300 (5 min) initially, increase to 3600 after verified

### Acceptance Criteria

- [ ] `server/nginx/blinkyeyes.com.conf` is syntactically valid Nginx config
- [ ] HTTP → HTTPS redirect is configured
- [ ] www → non-www redirect is configured
- [ ] SSL certificate paths are correct for Let's Encrypt
- [ ] Security headers include: X-Frame-Options, X-Content-Type-Options, HSTS, CSP, Referrer-Policy
- [ ] Gzip compression is enabled for text-based assets
- [ ] Static file caching is configured with appropriate expiry
- [ ] Custom 404 page is wired up
- [ ] Hidden files (dotfiles) are blocked
- [ ] `server/setup.sh` is executable and documents prerequisites
- [ ] `server/deploy.sh` is executable and uses rsync
- [ ] DNS requirements are documented

---

## Feature Block 6: Website Deployment Pipeline

**Purpose:** Automatically deploy the website to the px server whenever changes merge to main. No manual deploys.

**Depends on:** Block 3 (website exists), Block 5 (server is configured)
**Depended on by:** Block 8 (integration)

### What To Do

1. **Create `.github/workflows/deploy-site.yml`:**

   **Trigger:** On push to `main`, only when files in `website/` change.

   ```yaml
   on:
     push:
       branches: [main]
       paths:
         - 'website/**'
   ```

   **Job: `deploy`**
   - Runs on `ubuntu-latest`
   - Steps:
     1. Checkout code
     2. Install rsync (usually pre-installed on GitHub runners)
     3. Set up SSH key from secrets:
        ```yaml
        - name: Setup SSH
          run: |
            mkdir -p ~/.ssh
            echo "${{ secrets.DEPLOY_SSH_KEY }}" > ~/.ssh/id_ed25519
            chmod 600 ~/.ssh/id_ed25519
            ssh-keyscan -H ${{ secrets.DEPLOY_HOST }} >> ~/.ssh/known_hosts
        ```
     4. Deploy via rsync:
        ```yaml
        - name: Deploy website
          run: |
            rsync -avz --delete \
              --exclude '.DS_Store' \
              website/ ${{ secrets.DEPLOY_USER }}@${{ secrets.DEPLOY_HOST }}:/var/www/blinkyeyes.com/
        ```

   **Required GitHub Secrets (document in a comment block):**
   - `DEPLOY_SSH_KEY` — Private SSH key (ed25519) for the deploy user on px
   - `DEPLOY_HOST` — The server's IP or hostname
   - `DEPLOY_USER` — SSH username (e.g., `deploy` or `www-data`)

2. **Document secret setup** in a comment at the top of the workflow:
   ```
   # Required secrets:
   #   DEPLOY_SSH_KEY  - ed25519 private key for SSH access to the server
   #   DEPLOY_HOST     - Server hostname or IP
   #   DEPLOY_USER     - SSH user with write access to /var/www/blinkyeyes.com
   #
   # Setup:
   #   1. Generate a deploy key: ssh-keygen -t ed25519 -f deploy_key -N ""
   #   2. Add deploy_key.pub to the server's authorized_keys
   #   3. Add the private key content as DEPLOY_SSH_KEY secret in GitHub
   #   4. Add server hostname as DEPLOY_HOST
   #   5. Add SSH username as DEPLOY_USER
   ```

3. **Add a manual trigger** as a fallback:
   ```yaml
   on:
     push:
       branches: [main]
       paths:
         - 'website/**'
     workflow_dispatch:  # Allow manual trigger from GitHub UI
   ```

### Acceptance Criteria

- [ ] `.github/workflows/deploy-site.yml` is valid YAML
- [ ] Workflow triggers on push to main when website files change
- [ ] Workflow can also be triggered manually via workflow_dispatch
- [ ] SSH setup step uses secrets (not hardcoded values)
- [ ] rsync command syncs the `website/` directory to the server
- [ ] Required secrets are documented in the workflow file
- [ ] Deploy key setup instructions are documented

---

## Feature Block 7: App Distribution Polish

**Purpose:** Make the Blinky download and install experience feel professional. Proper icons, installer branding, and app metadata.

**Depends on:** Block 0
**Depended on by:** Block 8 (integration)

### What To Do

1. **App icons** — The current icons are solid-color placeholders. Create proper app icons:

   **Design:** A simple, recognizable eye icon. Stylized, not realistic. The eye should be:
   - Friendly and approachable (rounded shapes, not sharp)
   - Recognizable at small sizes (16x16 system tray)
   - Works on both light and dark backgrounds
   - Primary color: Blinky blue (`#3B82F6`)
   - Clean, flat design — no gradients, no 3D effects

   **Required sizes (update existing files in `src-tauri/icons/`):**
   - `icon.png` — 512x512 (or 1024x1024), used as the base for all conversions
   - `32x32.png` — Small icon
   - `128x128.png` — Medium icon
   - `128x128@2x.png` — Retina medium
   - `icon.icns` — macOS app icon bundle (contains multiple sizes)
   - `icon.ico` — Windows icon (contains 16, 32, 48, 256px sizes)
   - `Square30x30Logo.png` — Windows start menu
   - `Square44x44Logo.png` — Windows taskbar
   - `Square71x71Logo.png` — Windows medium tile
   - `Square89x89Logo.png` — Windows large tile start
   - `Square107x107Logo.png` — Windows tile
   - `Square142x142Logo.png` — Windows large tile
   - `Square150x150Logo.png` — Windows tile
   - `Square284x284Logo.png` — Windows extra large tile
   - `Square310x310Logo.png` — Windows extra large tile
   - `StoreLogo.png` — Windows store logo

   **Tray icons (update existing in `src-tauri/icons/`):**
   - `tray-default.png` — 22x22 + 44x44 @2x versions
   - `tray-active.png` — 22x22 + 44x44 @2x versions
   - `tray-paused.png` — 22x22 + 44x44 @2x versions

   **For MVP:** Generate programmatic icons using a script (Python PIL/Pillow, or even just SVG → PNG conversion). The key is that they look intentional, not placeholder. A simple eye shape (two arcs forming an almond shape with a circle in the center) in the right colors is sufficient.

2. **Update `tauri.conf.json`** metadata:
   - Verify `productName` is "Blinky"
   - Verify `identifier` is "com.blinky.app"
   - Add a `description` field if missing: "A gentle eye-rest reminder for the 20-20-20 rule"
   - Verify `version` matches what's in `package.json`
   - Ensure the `bundle` section references all icon files correctly
   - Add `homepage` URL: "https://blinkyeyes.com"

3. **Update `package.json`** metadata:
   - `name`: "blinky"
   - `version`: "1.0.0" (or "0.1.0" for initial release)
   - `description`: "A gentle eye-rest reminder for the 20-20-20 rule"
   - `homepage`: "https://blinkyeyes.com"
   - `repository`: GitHub repo URL
   - `license`: "MIT"
   - `author`: "Blinky Contributors"

4. **macOS DMG background** (optional enhancement):
   - Tauri supports custom DMG backgrounds. If time permits, create a simple background image that shows "Drag Blinky to Applications" with an arrow.
   - Configure in `tauri.conf.json` under `bundle.macOS.dmg`

5. **Linux `.desktop` file metadata:**
   - Tauri auto-generates this, but verify it includes:
     - `Name=Blinky`
     - `Comment=A gentle eye-rest reminder for the 20-20-20 rule`
     - `Categories=Utility;Health;`
     - `Keywords=eye;rest;20-20-20;health;timer;`

6. **Website assets from app:**
   - Copy/derive the app icon into `website/favicon.ico`, `website/favicon.svg`, and `website/apple-touch-icon.png`
   - Create `website/og-image.png` — A 1200x630 image with:
     - Clean background (white or light gradient)
     - Blinky logo/icon centered
     - "Blinky" text
     - Tagline: "Gentle eye-rest reminders"
     - This is what shows up when someone shares the link on Twitter/Slack/etc.

### Acceptance Criteria

- [ ] All icon files in `src-tauri/icons/` are valid PNG/ICO/ICNS with correct dimensions
- [ ] Tray icons are 22x22 (and 44x44 @2x) with transparent backgrounds
- [ ] `tauri.conf.json` has correct metadata (name, identifier, description, version)
- [ ] `package.json` has correct metadata (name, version, description, homepage, license)
- [ ] Website favicon files exist and are correctly sized
- [ ] `og-image.png` exists and is 1200x630
- [ ] Icons look intentional, not placeholder (recognizable eye shape)
- [ ] `cargo check` still passes after any changes to `tauri.conf.json`

---

## Feature Block 8: Integration & End-to-End Verification

**Purpose:** Verify that everything works together. The full pipeline: code → CI → release → website → server → downloads.

**Depends on:** All other blocks

### Verification Checklist

Go through each of these and verify:

| What | How to Verify |
|------|--------------|
| **README renders correctly** | Open on GitHub (or preview locally). All badges, links, and images work. |
| **CI workflow runs** | Push a PR and verify the CI job runs all checks (or dry-run the YAML locally with `act`). |
| **Release workflow is ready** | Verify the YAML is syntactically correct. Don't actually tag a release yet — just ensure the config is right. |
| **Website loads locally** | Open `website/index.html` in a browser. All sections render, no broken images or links. |
| **Website is responsive** | Test at 375px, 768px, and 1440px widths. Layout doesn't break. |
| **OS detection works** | Test the download button in different browser user-agent modes (Chrome DevTools device toolbar). |
| **Download links work** | Verify the GitHub Release URL pattern is correct (even if no release exists yet). |
| **SEO meta tags** | View page source and verify all OG, Twitter, and structured data tags are present. |
| **404 page** | Navigate to a non-existent URL (e.g., `website/404.html`) and verify it renders correctly. |
| **Nginx config is valid** | Run `nginx -t` on the config (if possible) or validate structure manually. |
| **Deploy script works** | Test `server/deploy.sh` (or verify rsync command syntax). |
| **Git history is clean** | All files are committed, no untracked files that should be tracked. |
| **No secrets in code** | Grep for API keys, passwords, or hardcoded IPs. All sensitive values use GitHub Secrets or environment variables. |
| **Favicon shows** | Open `website/index.html` in a browser tab — the favicon should appear. |
| **Social preview** | Use a social media debugger (Twitter Card Validator, Facebook Sharing Debugger URL patterns) to verify OG tags would work. |

### Pre-Launch Checklist

Before the first public release:

1. **DNS is configured** — `blinkyeyes.com` points to the px server
2. **SSL is active** — `https://blinkyeyes.com` works
3. **Website is deployed** — All files are on the server
4. **GitHub repo is public** — Visibility set to public
5. **First release is tagged** — `v0.1.0` or `v1.0.0`
6. **Download links work** — Each platform's download link resolves to a real file
7. **README badges are green** — CI passing, license badge showing

### Post-Launch Items (Document, Don't Implement)

These are nice-to-haves for after launch. Document them in a "Future Enhancements" section of the completion file:

- **Analytics:** Privacy-respecting analytics (Plausible, Umami, or simple server log analysis) to understand traffic
- **Auto-update:** Tauri's built-in updater for seamless in-app updates
- **Code signing:** macOS notarization and Windows Authenticode signing
- **CDN:** Cloudflare or similar in front of the server for global performance
- **Custom domain email:** hello@blinkyeyes.com for the GitHub profile
- **Homebrew cask:** `brew install --cask blinky` for macOS users
- **AUR package:** For Arch Linux users
- **Snap/Flatpak:** Additional Linux distribution formats

### Acceptance Criteria

- [ ] Every item in the verification checklist has been checked
- [ ] No broken links in any file
- [ ] No hardcoded secrets or IPs in committed code
- [ ] All workflows are valid YAML
- [ ] Website renders correctly at mobile, tablet, and desktop widths
- [ ] All files from the canonical file tree exist
- [ ] Pre-launch checklist items are documented (even if not all completed — they require infrastructure access)
- [ ] Post-launch items are documented in the completion file

---

## Parallelization Map

```
                       ┌──────────┐
                       │ Block 0  │  MUST BE FIRST
                       │  Repo    │
                       │  Setup   │
                       └────┬─────┘
                            │
        ┌───────┬───────┬───┼───────┐
        │       │       │   │       │
   ┌────▼──┐ ┌─▼────┐ ┌▼───▼──┐ ┌──▼───┐
   │Blk 1  │ │Blk 3 │ │Blk 5  │ │Blk 7 │
   │  CI   │ │ Land │ │Server │ │ App  │
   │Pipeln │ │ Page │ │Config │ │Distro│
   └──┬────┘ └──┬───┘ └──┬────┘ └──────┘
      │         │        │
   ┌──▼───┐ ┌──▼───┐    │
   │Blk 2 │ │Blk 4 │    │
   │Releas│ │ SEO  │    │
   │Pipeln│ │Polish│    │
   └──────┘ └──┬───┘    │
               │        │
            ┌──▼────────▼──┐
            │    Block 6   │
            │  Deploy      │
            │  Pipeline    │
            └──────┬───────┘
                   │
             ┌─────▼──────┐
             │  Block 8   │  MUST BE LAST
             │ Integration│
             └────────────┘
```

**Maximum parallelism: 4 blocks** after Block 0 is done. Blocks 1, 3, 5, and 7 have no dependencies on each other.

**Recommended sequential order** (if working linearly):
0 → 1 → 2 → 3 → 4 → 5 → 7 → 6 → 8

This ordering builds the infrastructure first (CI/release), then the website, then server config, and finally wires deployment together.

---

## Appendix A: GitHub Repository Details

**Repo name:** `blinky` (or `blinkyeyes` — use whatever feels right)
**Description:** "A gentle eye-rest reminder for the 20-20-20 rule. Free, open-source, cross-platform."
**Topics/tags:** `eye-health`, `20-20-20-rule`, `tauri`, `desktop-app`, `rust`, `react`, `eye-strain`, `health`, `utility`, `open-source`
**Homepage URL:** `https://blinkyeyes.com`
**License:** MIT

**Repository settings to configure (manually on GitHub):**
- Enable Issues
- Enable Discussions (optional — nice for community Q&A)
- Disable Wiki (README + website covers documentation)
- Default branch: `main`
- Branch protection on `main`: require CI to pass, require 1 review (if collaborators exist)

## Appendix B: Server Assumptions

The `px` server is a dedicated Linux server with:
- Nginx installed (or installable via `apt`)
- Root or sudo access
- SSH access configured (key-based auth)
- Certbot available or installable
- DNS already pointed (or will be pointed) to the server IP
- Sufficient disk space for static website files (~5MB)

The deploy user should have:
- Write access to `/var/www/blinkyeyes.com/`
- No root access (principle of least privilege)
- SSH key authentication (no password)

## Appendix C: Design Tokens

Use these consistently across the website and any new assets:

| Token | Value | Usage |
|-------|-------|-------|
| Primary blue | `#3B82F6` | CTAs, links, accents |
| Primary blue hover | `#2563EB` | Button hover states |
| Dark text | `#111827` | Headings (gray-900) |
| Body text | `#4B5563` | Paragraphs (gray-600) |
| Light text | `#9CA3AF` | Captions, footer (gray-400) |
| Background | `#FFFFFF` | Page background |
| Surface | `#F9FAFB` | Card backgrounds (gray-50) |
| Border | `#E5E7EB` | Subtle dividers (gray-200) |
| Success green | `#10B981` | Positive indicators |
| Font stack | `-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif` | All text |
| Border radius | `12px` / `0.75rem` | Cards and containers |
| Max content width | `1120px` / `70rem` | Page content constraint |

## Appendix D: Content Copy Guidelines

The website copy should be:
- **Conversational, not corporate.** "Rest your eyes" not "Optimize your ocular wellness."
- **Benefit-led.** "Your eyes will thank you" not "Implements the 20-20-20 rule algorithm."
- **Brief.** Every sentence earns its place. If a section can be said in 10 words, don't use 20.
- **Honest.** "Free forever" means free forever. No asterisks, no "for now."
- **Accessible.** No jargon. A non-technical person should understand everything on the page.

Avoid:
- Marketing buzzwords ("revolutionary", "cutting-edge", "game-changing")
- Exclamation marks (one or two max on the whole page)
- Technical implementation details on the landing page (save for README/GitHub)
- Comparisons to competitors
- Dark patterns or urgency tactics
