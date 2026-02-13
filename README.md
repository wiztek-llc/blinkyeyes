# Blinky

**A gentle eye-rest reminder for the 20-20-20 rule.**

[![CI](https://github.com/tekwiz/blinky/actions/workflows/ci.yml/badge.svg)](https://github.com/tekwiz/blinky/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-lightgrey)
[![Latest Release](https://img.shields.io/github/v/release/tekwiz/blinky)](https://github.com/tekwiz/blinky/releases/latest)

Blinky reminds you to follow the 20-20-20 rule — every 20 minutes, look at something 20 feet away for 20 seconds. Non-intrusive, lightweight, and completely free.

<!-- TODO: Replace with actual screenshot once available -->
![Blinky Dashboard](website/assets/screenshot-dashboard.png)

## Features

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

## Installation

Download the latest version from the [releases page](https://github.com/tekwiz/blinky/releases/latest).

| Platform | Instructions |
|----------|-------------|
| **macOS** | Open the `.dmg`, drag Blinky to Applications |
| **Windows** | Run the `.exe` installer |
| **Linux** | Download the `.AppImage`, `chmod +x`, and run. Also available as `.deb`. |

## Building from Source

Prerequisites: Node.js 18+, Rust 1.70+, and [Tauri v2 system dependencies](https://v2.tauri.app/start/prerequisites/).

```bash
npm install
npm run tauri dev      # Development
npm run tauri build    # Production build
```

## The 20-20-20 Rule

Every **20 minutes**, look at something **20 feet away** for **20 seconds**.

This simple habit reduces eye strain, dry eyes, and fatigue from prolonged screen use. It's recommended by the [American Academy of Ophthalmology](https://www.aao.org/eye-health/tips-prevention/computer-usage).

## Contributing

Contributions welcome! See our [contributing guide](CONTRIBUTING.md).

## License

[MIT](LICENSE)
