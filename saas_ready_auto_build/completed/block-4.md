# Block 4: Landing Page — Polish, SEO & Meta

## Files Created
- `website/404.html` — Custom 404 page with same nav/footer as main page, centered "Page not found" content, and "Go to homepage" button

## Files Modified
- `website/index.html` — Added Open Graph meta tags, Twitter Card meta tags, canonical URL, JSON-LD structured data, favicon references (already existed but now grouped under comment), and preconnect hint for Tailwind CDN

## Deviations from Spec
- **No `og-image.png` created.** The spec lists it in Block 4 but Block 7 (App Distribution Polish) is responsible for creating website assets derived from the app icon. The OG/Twitter meta tags reference `https://blinkyeyes.com/og-image.png` — Block 7 will create this file.
- **`robots.txt` and `sitemap.xml` unchanged.** Block 3 already created these with the correct content per the Block 4 spec. No modifications needed.
- **No `loading="lazy"` added.** The page contains no `<img>` tags — all visuals are inline SVGs (decorative, with `aria-hidden="true"`) or CSS-styled `<div>` placeholders. Lazy loading is not applicable.
- **No inline critical CSS added for Tailwind CDN FOUC.** The Tailwind CDN play script loads synchronously via `<script>`, so it blocks rendering and no FOUC occurs. A `<link rel="preconnect">` hint was added instead to speed up the CDN connection.
- **`softwareVersion` set to `0.1.0`** in JSON-LD, matching the current version in `package.json` and `tauri.conf.json` (as noted in Block 0 decisions).
- **404 page includes `<meta name="robots" content="noindex">`** to prevent search engines from indexing the error page.

## Acceptance Criteria Results
- [x] All Open Graph meta tags are present and valid (`og:type`, `og:url`, `og:title`, `og:description`, `og:image`)
- [x] All Twitter Card meta tags are present (`twitter:card`, `twitter:title`, `twitter:description`, `twitter:image`)
- [x] Canonical URL points to `https://blinkyeyes.com/`
- [x] Favicon is referenced correctly in multiple formats (`.ico`, `.svg`, `apple-touch-icon`)
- [x] `robots.txt` allows all crawlers and references sitemap (created in Block 3, unchanged)
- [x] `sitemap.xml` is valid XML with the correct URL (created in Block 3, unchanged)
- [x] `404.html` exists and is styled consistently with the main page (same Tailwind config, design tokens, nav, footer)
- [x] JSON-LD structured data is valid (SoftwareApplication schema with correct fields)
- [x] All images have alt text (no `<img>` tags — all visuals are inline SVGs with `aria-hidden="true"`)
- [x] Page uses semantic HTML elements (`<header>`, `<nav>`, `<main>`, `<section>`, `<footer>`)
- [x] Theme color meta tag is present (`#3B82F6`)

## Known Issues / TODOs for Later Blocks
- **Block 7** should create `og-image.png` (1200x630) — the OG/Twitter meta tags already reference it
- The Tailwind CDN play script should eventually be replaced with a pre-compiled CSS file for production (documented in Block 3 decisions)
- The 404 page uses the Tailwind CDN separately; if Tailwind is optimized for the main page, the 404 page should be updated to match
