# Decisions Log

## Block 0: Repository Setup & GitHub Configuration

- **GitHub owner/repo**: Used `tekwiz/blinky` as the owner/repo for all badge URLs, release links, and issue template links. No git remote was configured at the time of implementation — update these if the actual repo path differs.
- **Version**: Kept `0.1.0` from the existing `package.json` and `tauri.conf.json` rather than bumping to `1.0.0`. This is a pre-release state; the first tagged release can be `v0.1.0`.
- **Copyright year**: Used 2026 (current year) with "Blinky Contributors" as the copyright holder per the spec.
- **.idea/ in .gitignore**: Added `.idea/` to `.gitignore`. Some `.idea/` files were already staged in the initial commit state — these should be unstaged/removed from tracking when convenient (e.g., `git rm --cached -r .idea/`).
- **.DS_Store in staging**: `.DS_Store` was already staged. Should be removed from tracking (`git rm --cached .DS_Store`) since it's in `.gitignore`.
- **README screenshot placeholder**: The hero screenshot `website/assets/screenshot-dashboard.png` doesn't exist yet. Added a TODO comment. This will be resolved in Block 3 or Block 7.
- **Tauri v2 prerequisites link**: Used the official Tauri v2 prerequisites page in the "Building from Source" section rather than listing all system deps inline, since they vary by OS and Tauri version.

## Block 1: CI Pipeline (Lint, Test, Type-Check)

- **Cargo working directory**: All `cargo` commands use `working-directory: src-tauri` because the Cargo workspace is rooted there, not at the repo root. This is cleaner than passing `--manifest-path` to every command.
- **Node.js caching via setup-node**: Used `actions/setup-node@v4`'s built-in `cache: npm` feature instead of a separate `actions/cache` step for node_modules. This caches the npm global store and lets `npm ci` restore efficiently. It's simpler and the recommended approach.
- **Cargo caching keyed on Cargo.lock**: The Cargo cache key uses `hashFiles('src-tauri/Cargo.lock')` since that's where the lockfile lives. Caches `~/.cargo/registry`, `~/.cargo/git`, and `src-tauri/target`.
- **Rustfmt and Clippy via dtolnay/rust-toolchain**: Installed `rustfmt` and `clippy` components via the `dtolnay/rust-toolchain@stable` action's `components` field rather than separate `rustup component add` steps.
- **Single Ubuntu runner**: The spec only calls for one `check` job on `ubuntu-latest`. Cross-platform CI was not added — the release pipeline (Block 2) handles multi-platform builds.

## Block 2: Release Pipeline (Cross-Platform Build + GitHub Releases)

- **`fail-fast: false`**: Set to `false` so all platform builds continue even if one fails. For a release, partial artifact availability (e.g., Linux and Windows succeed but macOS fails) is better than no artifacts at all.
- **macOS universal binary via `rust_targets` matrix field**: The macOS matrix entry includes `rust_targets: aarch64-apple-darwin,x86_64-apple-darwin` which is passed to `dtolnay/rust-toolchain`'s `targets` parameter. This installs both targets, and the Tauri action builds a fat binary via `--target universal-apple-darwin`. Non-macOS entries omit `rust_targets`, so only the default host target is installed.
- **Cargo cache key prefix `release`**: Used `${{ runner.os }}-cargo-release-` as the cache key to separate release build caches from CI caches. Release builds may differ (e.g., optimized builds vs. debug) so they shouldn't share the target directory. The `restore-keys` fallback includes the base `cargo-` prefix so a CI cache can warm a release build.
- **Code signing as commented env vars**: macOS and Windows signing secrets are documented both in the top comment block and as commented-out `env` entries in the Tauri action step. This makes it trivial to enable — just uncomment and add the GitHub Secrets.
- **CONTRIBUTING.md changelog guidance**: Added a single line about moving `[Unreleased]` items to a versioned heading. The existing semver note was already present from Block 0, so only the changelog workflow detail was added.

## Block 3: Landing Page — Structure & Content

- **Tailwind via CDN play script**: Used `<script src="https://cdn.tailwindcss.com">` as the spec explicitly allows for MVP. This adds ~300KB of JS at runtime. Should be replaced with a pre-compiled CSS file (via Tailwind CLI standalone) before high-traffic launch. Documented as a known optimization.
- **No separate styles.css**: All styling is handled by Tailwind utility classes in the HTML plus a small inline `<style>` block for fade-in transitions and smooth scroll. A separate CSS file would only contain these few lines and adds an unnecessary HTTP request. Block 4 may introduce additional styles if needed.
- **Inline SVG icons instead of images**: All icons (platform logos, feature icons, eye icon in hero) are inline SVGs rather than external image files. This eliminates HTTP requests, avoids broken image states, and allows Tailwind color utilities to style them. All decorative SVGs use `aria-hidden="true"`.
- **Hero placeholder as styled div**: The hero screenshot is a CSS-styled `<div>` that looks like a macOS-style app window (traffic light dots, title bar). This is intentional per the spec — "If you can't generate proper screenshots/mockups, create well-styled placeholder `<div>` elements."
- **Favicon generation**: Used Python/Pillow to render eye icons at 16/32/48px, then ImageMagick `convert` to create a proper multi-size `.ico` file. Pillow's native ICO writer only embeds the first size; ImageMagick correctly bundles all three.
- **All download links point to `/releases/latest`**: Platform-specific deep links to individual assets (e.g., `.dmg`, `.exe`, `.AppImage`) would require knowing exact filenames, which depend on the version and aren't available until a release is tagged. The `/releases/latest` page shows all platform downloads.
- **Skip-to-content link**: Added a visually hidden "Skip to content" link for screen reader accessibility, using Tailwind's `sr-only` / `focus:not-sr-only` utilities. The `<main id="main">` element is the skip target.
- **IntersectionObserver fallback**: The fade-in animation includes a fallback for browsers that don't support `IntersectionObserver` — all elements are shown immediately. This prevents content from being permanently hidden in older browsers.
- **Privacy in footer**: Rather than creating a separate privacy page (which the spec doesn't call for), the footer has inline text with a tooltip: "Privacy: We collect nothing." This matches the spec's suggestion to "link to section or page explaining 'we collect nothing'."

## Block 4: Landing Page — Polish, SEO & Meta

- **OG/Twitter meta tags reference og-image.png before it exists**: The meta tags point to `https://blinkyeyes.com/og-image.png` which doesn't exist yet. Block 7 (App Distribution Polish) is responsible for creating this file. Social preview debuggers will show a broken image until then — this is intentional to avoid duplicating work.
- **JSON-LD version matches package.json**: Used `softwareVersion: "0.1.0"` to match the current version set in Block 0 decisions. This should be updated when the first release is tagged.
- **Preconnect instead of inline critical CSS**: Added `<link rel="preconnect" href="https://cdn.tailwindcss.com">` rather than inlining critical CSS. The Tailwind CDN script loads synchronously (it's a `<script>` tag, not `<link rel="stylesheet">`), so it blocks rendering and there's no flash of unstyled content. The preconnect hint saves ~100ms on the TLS handshake.
- **404 page marked noindex**: Added `<meta name="robots" content="noindex">` to prevent search engines from indexing the 404 page. This is standard practice for error pages.
- **404 nav is always opaque**: Unlike the main page where the nav starts transparent and becomes opaque on scroll, the 404 page nav is always opaque (`bg-white/80 backdrop-blur-md border-b`). This is simpler and there's no content to scroll through above the fold.
- **No changes to robots.txt or sitemap.xml**: Block 3 already created these files with content that exactly matches the Block 4 spec. No modifications needed.

## Block 5: Server Configuration (Nginx, SSL, Security)

- **Nginx config follows spec exactly**: The three-server-block structure (HTTP redirect, main HTTPS, www HTTPS redirect) matches the spec verbatim. No deviations or additions were needed.
- **CSP allows Tailwind CDN**: The Content-Security-Policy header includes `https://cdn.tailwindcss.com` in `script-src` to match the current Tailwind CDN play script used in Block 3. If Tailwind is later compiled to a static CSS file, this CSP entry should be removed to tighten security.
- **YOUR_EMAIL placeholder in setup.sh**: Left as `YOUR_EMAIL` per the spec's instruction to "Replace YOUR_EMAIL with the actual email. Add a comment noting this." A TODO comment marks the line. Must be replaced before running on the production server.
- **No nginx -t validation**: Nginx is not installed on the development machine, so `nginx -t` could not be run. The config was manually reviewed against Nginx documentation and follows standard conventions. Validation should be done on the px server during setup.
- **deploy.sh uses `px` as SSH host alias**: The deploy script uses `px` as the remote host, matching the spec's reference to `ssh px`. This assumes the deployer has an SSH config entry for `px` pointing to the server.

## Block 7: App Distribution Polish

- **Icon design**: Programmatically generated using Python/Pillow. The icon is a stylized eye (almond shape with iris, pupil, and highlight dot) on a blue (#3B82F6) rounded-rectangle background. The design is clean, flat, and recognizable at all sizes down to 16x16. A designer could later refine the icon while keeping the same shape and color scheme.
- **icon.png at 1024x1024**: Used the larger maximum size (spec said "512x512 or 1024x1024") to maximize quality when downscaling to all other sizes. All smaller sizes are derived from this base via Lanczos resampling.
- **Proper ICO via ImageMagick**: The `.ico` file contains 4 sizes (16, 32, 48, 256) at 32-bit RGBA. Previous blocks had a single PNG renamed to `.ico` — now it's a proper multi-size Windows icon container.
- **Proper ICNS via png2icns**: Installed `icnsutils` to generate a valid macOS `.icns` bundle with 7 sizes (16, 32, 48, 128, 256, 512, 1024). ImageMagick's ICNS support on Linux produces invalid output (just a renamed PNG), so `png2icns` was required.
- **Tray icon design consistency**: Tray icons use the same eye motif as the app icon but without the rounded-rect background (transparent). Default (blue), active (green #10B981), and paused (gray #9CA3AF with squinting eye + pause bars) states are visually distinct.
- **Windows Square logos from `icons/` directory**: Generated all 10 Windows-specific sizes (Square30x30Logo through Square310x310Logo plus StoreLogo). These are not listed in `bundle.icon` in `tauri.conf.json` because Tauri's NSIS/MSI builder discovers them from the `icons/` directory by filename convention.
- **og-image.png**: Created a clean 1200x630 social share image with the app icon, "Blinky" title, tagline, URL, and platform badges. Uses DejaVu Sans font (available on the build machine). This resolves the placeholder noted in Block 4's decisions.
- **favicon.svg updated**: Changed from a bare eye shape to match the app icon (eye on rounded-rect background) for visual consistency across the website favicon and app icon.
- **No DMG background**: The spec marks this as "optional enhancement". Skipped for MVP — can be added later via `bundle.macOS.dmg` in `tauri.conf.json`.
- **Linux .desktop Keywords not set**: Tauri v2 doesn't expose the `Keywords` field through `tauri.conf.json`. The auto-generated `.desktop` file will have `Name=Blinky`, `Comment=A gentle eye-rest reminder...`, and `Categories=Utility;` from the config fields. Adding `Keywords` would require a custom desktop template, which is a future enhancement.
- **Version kept at 0.1.0**: Maintained consistency with Block 0's decision. Both `tauri.conf.json` and `package.json` show `0.1.0`.

## Block 6: Website Deployment Pipeline

- **No rsync install step**: rsync is pre-installed on `ubuntu-latest` GitHub runners, so no explicit installation step was needed. This keeps the workflow minimal.
- **Workflow name "Deploy Website"**: Named distinctly from the CI and Release workflows so it's easy to identify in the GitHub Actions UI.
- **rsync excludes match deploy.sh**: The rsync command excludes `.DS_Store` and `*.map` files, matching the manual `server/deploy.sh` script from Block 5 for consistency.
- **No concurrency control**: The spec doesn't mention it, and for a simple static site deploy, concurrent deploys are unlikely. If rapid successive pushes cause issues, a `concurrency` group can be added later.

## Block 8: Integration & End-to-End Verification

- **Verification approach**: All checks were performed programmatically where possible — YAML validation via Python `yaml.safe_load`, XML validation via `xml.etree.ElementTree.parse`, JSON-LD extraction and validation via `json.loads`, image dimension verification via Pillow, and secrets scanning via regex grep. Manual structural review was used for HTML semantics, Tailwind responsive classes, and Nginx config syntax.
- **No live browser testing**: Responsive layout was verified by auditing Tailwind breakpoint classes (`sm:`, `md:`, `lg:`) and grid/flex patterns in the HTML source. Actual pixel-width testing at 375px/768px/1440px would require a browser — recommended as a manual step before launch.
- **No nginx -t**: Consistent with Block 5's decision. The Nginx config should be validated on the px server during initial setup.
- **Git staging deferred**: Many files from all blocks are currently untracked. The recommended workflow is: `git rm --cached .DS_Store .idea/.gitignore`, then `git add` all new files, then commit. This was not done during verification to avoid modifying the working tree state mid-audit.
- **FUNDING.yml omitted**: The spec lists `.github/FUNDING.yml` as "Optional: sponsor links (can be empty)". No funding information was available, so the file was not created. Can be added later if sponsorship is set up.
- **Screenshot placeholders remain**: `website/assets/screenshot-dashboard.png` and `screenshot-overlay.png` are referenced in the spec's file tree but were never created. The README uses a TODO comment; the website uses a styled CSS placeholder. These should be captured from the actual running app before launch.
