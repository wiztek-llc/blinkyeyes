use crate::state::{AppState, TimerPhase};
use tauri::{
    image::Image,
    menu::{IsMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    AppHandle, Manager,
};

const TRAY_ID: &str = "blinky_tray";

// Embedded tray icons (22x22 RGBA PNGs)
const ICON_DEFAULT_BYTES: &[u8] = include_bytes!("../icons/tray-default.png");
const ICON_ACTIVE_BYTES: &[u8] = include_bytes!("../icons/tray-active.png");
const ICON_PAUSED_BYTES: &[u8] = include_bytes!("../icons/tray-paused.png");

/// Holds menu items that need dynamic text updates.
/// Managed as Tauri state so `update_tray_status` can access them.
pub struct TrayMenuState {
    pub status_item: MenuItem<tauri::Wry>,
    pub pause_resume_item: MenuItem<tauri::Wry>,
}

fn format_time(seconds: u64) -> String {
    let m = seconds / 60;
    let s = seconds % 60;
    format!("{m:02}:{s:02}")
}

/// Create the system tray icon and context menu.
/// Returns `TrayMenuState` to be `.manage()`d by Tauri.
pub fn create_tray(app: &AppHandle) -> Result<TrayMenuState, Box<dyn std::error::Error>> {
    // Build menu items
    let status_item =
        MenuItem::with_id(app, "status", "Next break in 20:00", false, None::<&str>)?;
    let pause_item = MenuItem::with_id(app, "pause_resume", "Pause", true, None::<&str>)?;
    let skip_item = MenuItem::with_id(app, "skip_break", "Skip Break", true, None::<&str>)?;
    let reset_item = MenuItem::with_id(app, "reset_timer", "Reset Timer", true, None::<&str>)?;
    let dashboard_item =
        MenuItem::with_id(app, "open_dashboard", "Open Dashboard", true, None::<&str>)?;
    let settings_item = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let quit_item = MenuItem::with_id(app, "quit", "Quit Blinky", true, None::<&str>)?;

    let sep1 = PredefinedMenuItem::separator(app)?;
    let sep2 = PredefinedMenuItem::separator(app)?;
    let sep3 = PredefinedMenuItem::separator(app)?;

    let menu = Menu::with_items(
        app,
        &[
            &status_item as &dyn IsMenuItem<tauri::Wry>,
            &sep1,
            &pause_item,
            &skip_item,
            &reset_item,
            &sep2,
            &dashboard_item,
            &settings_item,
            &sep3,
            &quit_item,
        ],
    )?;

    let icon = Image::from_bytes(ICON_DEFAULT_BYTES)?;

    let _tray = TrayIconBuilder::with_id(TRAY_ID)
        .icon(icon)
        .tooltip("Blinky — Next break in 20:00")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| {
            handle_menu_event(app, &event);
        })
        .on_tray_icon_event(|tray, event| {
            handle_tray_event(tray.app_handle(), &event);
        })
        .build(app)?;

    Ok(TrayMenuState {
        status_item,
        pause_resume_item: pause_item,
    })
}

fn handle_menu_event(app: &AppHandle, event: &MenuEvent) {
    match event.id().as_ref() {
        "pause_resume" => {
            let state = app.state::<AppState>();
            let phase = state.timer.lock().unwrap().phase.clone();
            if phase == TimerPhase::Paused {
                crate::timer::resume(app);
            } else {
                crate::timer::pause(app);
            }
        }
        "skip_break" => {
            crate::timer::skip_break(app);
        }
        "reset_timer" => {
            crate::timer::reset(app);
        }
        "open_dashboard" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "settings" => {
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
        "quit" => {
            app.exit(0);
        }
        _ => {}
    }
}

fn handle_tray_event(app: &AppHandle, event: &TrayIconEvent) {
    if let TrayIconEvent::Click {
        button: MouseButton::Left,
        button_state: MouseButtonState::Up,
        ..
    } = event
    {
        if let Some(window) = app.get_webview_window("main") {
            if window.is_visible().unwrap_or(false) {
                let _ = window.hide();
            } else {
                let _ = window.show();
                let _ = window.set_focus();
            }
        }
    }
}

/// Update the tray icon, tooltip, and menu status text.
/// Called every second by the timer loop. Must be cheap.
pub fn update_tray_status(app: &AppHandle, phase: &TimerPhase, seconds_remaining: u64) {
    // Update tooltip
    let tooltip = match phase {
        TimerPhase::Working => format!("Blinky — Next break in {}", format_time(seconds_remaining)),
        TimerPhase::Breaking => format!("Blinky — Look away! {}s remaining", seconds_remaining),
        TimerPhase::Paused => "Blinky — Paused".to_string(),
        TimerPhase::Suspended => "Blinky — Suspended (idle)".to_string(),
    };

    if let Some(tray) = app.tray_by_id(TRAY_ID) {
        let _ = tray.set_tooltip(Some(&tooltip));

        // Swap icon based on phase
        let icon_bytes = match phase {
            TimerPhase::Working => ICON_DEFAULT_BYTES,
            TimerPhase::Breaking => ICON_ACTIVE_BYTES,
            TimerPhase::Paused | TimerPhase::Suspended => ICON_PAUSED_BYTES,
        };
        if let Ok(icon) = Image::from_bytes(icon_bytes) {
            let _ = tray.set_icon(Some(icon));
        }
    }

    // Update dynamic menu item text
    if let Some(tray_state) = app.try_state::<TrayMenuState>() {
        let status_text = match phase {
            TimerPhase::Working => {
                format!("Next break in {}", format_time(seconds_remaining))
            }
            TimerPhase::Breaking => format!("Break — {}s remaining", seconds_remaining),
            TimerPhase::Paused => "Paused".to_string(),
            TimerPhase::Suspended => "Suspended (idle)".to_string(),
        };
        let _ = tray_state.status_item.set_text(&status_text);

        let pause_text = if *phase == TimerPhase::Paused {
            "Resume"
        } else {
            "Pause"
        };
        let _ = tray_state.pause_resume_item.set_text(pause_text);
    }
}
