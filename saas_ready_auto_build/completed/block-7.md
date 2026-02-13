# Block 7: App Distribution Polish

## Files Created
- `src-tauri/icons/icon.png` — 1024x1024 app icon (eye on blue rounded-rect background)
- `src-tauri/icons/32x32.png` — 32x32 small app icon
- `src-tauri/icons/128x128.png` — 128x128 medium app icon
- `src-tauri/icons/128x128@2x.png` — 256x256 retina medium app icon
- `src-tauri/icons/icon.ico` — Windows icon (multi-size: 16, 32, 48, 256 @ 32-bit RGBA)
- `src-tauri/icons/icon.icns` — macOS icon bundle (16, 32, 48, 128, 256, 512, 1024)
- `src-tauri/icons/Square30x30Logo.png` — Windows start menu
- `src-tauri/icons/Square44x44Logo.png` — Windows taskbar
- `src-tauri/icons/Square71x71Logo.png` — Windows medium tile
- `src-tauri/icons/Square89x89Logo.png` — Windows large tile start
- `src-tauri/icons/Square107x107Logo.png` — Windows tile
- `src-tauri/icons/Square142x142Logo.png` — Windows large tile
- `src-tauri/icons/Square150x150Logo.png` — Windows tile
- `src-tauri/icons/Square284x284Logo.png` — Windows extra large tile
- `src-tauri/icons/Square310x310Logo.png` — Windows extra large tile
- `src-tauri/icons/StoreLogo.png` — Windows store logo (50x50)
- `src-tauri/icons/tray-default.png` — 22x22 blue eye tray icon
- `src-tauri/icons/tray-default@2x.png` — 44x44 blue eye tray icon
- `src-tauri/icons/tray-active.png` — 22x22 green eye tray icon
- `src-tauri/icons/tray-active@2x.png` — 44x44 green eye tray icon
- `src-tauri/icons/tray-paused.png` — 22x22 gray squinting eye with pause bars
- `src-tauri/icons/tray-paused@2x.png` — 44x44 gray squinting eye with pause bars
- `website/og-image.png` — 1200x630 social share image

## Files Modified
- `src-tauri/tauri.conf.json` — Added shortDescription, longDescription, homepage, category, linux.deb config
- `package.json` — Added description, homepage, repository, license, author
- `website/favicon.ico` — Regenerated with proper multi-size ICO from new app icon
- `website/favicon.svg` — Updated to match app icon design (rounded-rect background + eye)
- `website/apple-touch-icon.png` — Regenerated from new app icon (180x180)
- `website/assets/logo.svg` — Updated eye icon to match app icon design

## Deviations from Spec
- **icon.png is 1024x1024** (spec says "512x512 or 1024x1024") — used the larger size for maximum quality when downscaling.
- **No macOS DMG background** — Marked as "optional enhancement" in the spec. Skipped for MVP; can be added later. Tauri v2's DMG background configuration would be in `bundle.macOS.dmg` but the visual asset would need design work.
- **Linux `.desktop` metadata** — Tauri auto-generates the `.desktop` file from `tauri.conf.json` fields. The `shortDescription` maps to `Comment=`, `productName` maps to `Name=`, and `category` maps to `Categories=`. The `Keywords` field is not directly configurable through `tauri.conf.json` in v2; it would require a custom desktop template. Left as a future enhancement.
- **Windows Square logos not referenced in bundle.icon** — The `bundle.icon` array only references the standard 5 icons that Tauri uses directly. The Windows Square logos are picked up automatically by the NSIS/MSI build process from the `icons/` directory based on filename convention.

## Acceptance Criteria Results
- [x] All icon files in `src-tauri/icons/` are valid PNG/ICO/ICNS with correct dimensions
- [x] Tray icons are 22x22 (and 44x44 @2x) with transparent backgrounds
- [x] `tauri.conf.json` has correct metadata (name: Blinky, identifier: com.blinky.app, description, version: 0.1.0)
- [x] `package.json` has correct metadata (name: blinky, version: 0.1.0, description, homepage, license: MIT)
- [x] Website favicon files exist and are correctly sized (favicon.ico multi-size, favicon.svg, apple-touch-icon 180x180)
- [x] `og-image.png` exists and is 1200x630
- [x] Icons look intentional, not placeholder (recognizable eye shape on blue rounded-rect background)
- [ ] `cargo check` still passes after changes to `tauri.conf.json` — Cannot run on this machine (no Rust toolchain in dev environment). The JSON is valid and only added standard Tauri v2 bundle fields.

## Known Issues / TODOs for Later Blocks
- **`cargo check` not verified** — Rust toolchain not available in the current dev environment. The `tauri.conf.json` changes only add standard bundle metadata fields and should not affect compilation.
- **macOS DMG background** — Not created. Can be added as a post-launch enhancement.
- **Linux `.desktop` Keywords** — Tauri v2 doesn't expose Keywords in `tauri.conf.json`. Would require a custom desktop template for `Keywords=eye;rest;20-20-20;health;timer;`.
- **Icon generation is programmatic** — Icons were generated via Python/Pillow with a simple eye shape. A designer could later refine the icon with more detail while keeping the same general shape and colors.
