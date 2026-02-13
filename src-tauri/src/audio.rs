// Audio playback has been moved to the frontend (WebView) for cross-platform
// compatibility. The backend emits `break-completed` events; the frontend
// plays the chime via the Web Audio API. This module is kept as a stub so
// that `mod audio` in lib.rs still resolves.
