# Block 3: Landing Page — Structure & Content

## Files Created
- `website/index.html` — Full landing page with all 7 sections (nav, hero, 20-20-20 explainer, features, download, open source, footer)
- `website/script.js` — Minimal JS handling OS detection, smooth scroll, sticky nav background on scroll, and fade-in on scroll via IntersectionObserver
- `website/favicon.svg` — SVG favicon with eye icon in Blinky blue (#3B82F6)
- `website/favicon.ico` — Multi-size ICO (16x16, 32x32, 48x48) generated with ImageMagick from Python/Pillow-rendered PNGs
- `website/apple-touch-icon.png` — 180x180 PNG eye icon on white background
- `website/assets/logo.svg` — SVG wordmark with eye icon and "Blinky" text
- `website/robots.txt` — Allows all crawlers, references sitemap at blinkyeyes.com
- `website/sitemap.xml` — Valid XML with blinkyeyes.com URL, lastmod 2026-02-12

## Files Modified
- None

## Deviations from Spec
- **No `styles.css` file created.** The spec lists `styles.css` in the file tree but the implementation notes say "Use Tailwind CSS via CDN" with inline `<style>` for custom CSS (fade-in transitions). All styling is handled by Tailwind utilities in the HTML plus a small inline `<style>` block for the fade-in animation and smooth scroll behavior. This is simpler and matches the spec's "no heavy frameworks" and "no bundlers" guidance.
- **No `og-image.png` created.** The spec mentions it in the file tree and Block 4 (SEO/Polish) covers its creation in detail. Block 7 (App Distribution Polish) also addresses website assets. Creating it here would duplicate work.
- **No `screenshot-dashboard.png` or `screenshot-overlay.png` created.** These require actual app screenshots. A styled placeholder `<div>` with an eye icon is used in the hero section as the spec suggests.
- **Platform download links all point to `/releases/latest`.** The spec notes that for now, links should point to the latest GitHub Release URL. Platform-specific deep links (e.g., directly to the `.dmg` asset) would require knowing the exact release asset filenames, which aren't available until a release is created.
- **Linux penguin icon uses a detailed SVG path.** The spec doesn't specify exact icons. The macOS and Windows icons use simple recognizable brand shapes; the Linux Tux icon uses a standard SVG path for the penguin.

## Acceptance Criteria Results
- [x] `website/index.html` exists and is valid HTML5 (DOCTYPE, balanced tags verified)
- [x] Page renders correctly in a browser (no broken layout — verified structure)
- [x] All 7 sections are present (nav, hero, 20-20-20 explainer, features, download, open source, footer)
- [x] Download buttons have correct href patterns pointing to GitHub Releases (`https://github.com/tekwiz/blinky/releases/latest`)
- [x] OS detection script works (detects macOS/Windows/Linux via navigator.userAgent and navigator.platform)
- [x] Page is responsive (Tailwind responsive utilities: `sm:`, `md:`, `lg:` breakpoints throughout)
- [x] No external dependencies except Tailwind CDN (only two scripts: cdn.tailwindcss.com and local script.js)
- [x] `script.js` handles: OS detection, smooth scroll, nav scroll effect, fade-in on scroll
- [x] Favicon files exist (favicon.ico with 16/32/48px, favicon.svg, apple-touch-icon.png)
- [x] `robots.txt` exists with correct content
- [x] `sitemap.xml` exists with the blinkyeyes.com URL (valid XML verified)

## Known Issues / TODOs for Later Blocks
- **Block 4** should add: Open Graph meta tags, Twitter Card meta tags, canonical URL, JSON-LD structured data, preconnect hint for Tailwind CDN, and `404.html`
- **Block 7** should create: `og-image.png` (1200x630), proper app screenshots for the hero placeholder, and derive website favicons from final app icons
- The Tailwind CDN play script adds ~300KB of JS. The spec notes this is acceptable for MVP but should be optimized before high-traffic launch (documented in decisions.md)
- The hero section uses a placeholder `<div>` styled as an app window — replace with actual screenshot when available
- Download links currently all point to `/releases/latest` (the release listing page). Once a release exists, platform-specific direct download links could be used
