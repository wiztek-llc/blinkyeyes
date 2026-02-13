use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;

/// Send a notification when a break starts.
pub fn send_break_notification(app: &AppHandle) {
    if let Err(e) = app
        .notification()
        .builder()
        .title("Time for a break! 👀")
        .body("Look at something 20 feet away for 20 seconds.")
        .show()
    {
        eprintln!("[notifications] Failed to send break notification: {}", e);
    }
}

/// Send a notification when a break completes.
pub fn send_break_complete_notification(app: &AppHandle) {
    if let Err(e) = app
        .notification()
        .builder()
        .title("Break complete ✓")
        .body("Nice work! Your eyes will thank you.")
        .show()
    {
        eprintln!(
            "[notifications] Failed to send break-complete notification: {}",
            e
        );
    }
}
