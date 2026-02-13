/// Returns the system idle time in seconds (time since last keyboard/mouse input).
/// Returns None if idle detection is unavailable on this platform.
pub fn get_idle_seconds() -> Option<u64> {
    platform::get_idle_seconds()
}

// ---- Linux: dynamically load libX11 + libXss to avoid compile-time dependencies ----

#[cfg(target_os = "linux")]
mod platform {
    use std::os::raw::{c_char, c_int, c_ulong, c_void};
    use std::sync::OnceLock;

    #[repr(C)]
    struct XScreenSaverInfo {
        window: c_ulong,
        state: c_int,
        kind: c_int,
        since: c_ulong,
        idle: c_ulong,
        event_mask: c_ulong,
    }

    const RTLD_LAZY: c_int = 1;

    extern "C" {
        fn dlopen(filename: *const c_char, flags: c_int) -> *mut c_void;
        fn dlsym(handle: *mut c_void, symbol: *const c_char) -> *mut c_void;
    }

    type OpenDisplayFn = unsafe extern "C" fn(*const c_char) -> *mut c_void;
    type DefaultRootWindowFn = unsafe extern "C" fn(*mut c_void) -> c_ulong;
    type CloseDisplayFn = unsafe extern "C" fn(*mut c_void) -> c_int;
    type XFreeFn = unsafe extern "C" fn(*mut c_void) -> c_int;
    type AllocInfoFn = unsafe extern "C" fn() -> *mut XScreenSaverInfo;
    type QueryInfoFn = unsafe extern "C" fn(*mut c_void, c_ulong, *mut XScreenSaverInfo) -> c_int;

    struct IdleFns {
        open_display: OpenDisplayFn,
        default_root_window: DefaultRootWindowFn,
        close_display: CloseDisplayFn,
        x_free: XFreeFn,
        alloc_info: AllocInfoFn,
        query_info: QueryInfoFn,
    }

    // SAFETY: Function pointers from dlsym are valid for the process lifetime.
    // The loaded libraries are intentionally never dlclose'd.
    unsafe impl Send for IdleFns {}
    unsafe impl Sync for IdleFns {}

    static IDLE_FNS: OnceLock<Option<IdleFns>> = OnceLock::new();

    fn load_fns() -> Option<IdleFns> {
        unsafe {
            let x11 = dlopen(c"libX11.so.6".as_ptr(), RTLD_LAZY);
            if x11.is_null() {
                return None;
            }

            let xss = dlopen(c"libXss.so.1".as_ptr(), RTLD_LAZY);
            if xss.is_null() {
                return None;
            }

            let ptrs = [
                dlsym(x11, c"XOpenDisplay".as_ptr()),
                dlsym(x11, c"XDefaultRootWindow".as_ptr()),
                dlsym(x11, c"XCloseDisplay".as_ptr()),
                dlsym(x11, c"XFree".as_ptr()),
                dlsym(xss, c"XScreenSaverAllocInfo".as_ptr()),
                dlsym(xss, c"XScreenSaverQueryInfo".as_ptr()),
            ];

            if ptrs.iter().any(|p| p.is_null()) {
                return None;
            }

            Some(IdleFns {
                open_display: std::mem::transmute::<*mut c_void, OpenDisplayFn>(ptrs[0]),
                default_root_window: std::mem::transmute::<*mut c_void, DefaultRootWindowFn>(
                    ptrs[1],
                ),
                close_display: std::mem::transmute::<*mut c_void, CloseDisplayFn>(ptrs[2]),
                x_free: std::mem::transmute::<*mut c_void, XFreeFn>(ptrs[3]),
                alloc_info: std::mem::transmute::<*mut c_void, AllocInfoFn>(ptrs[4]),
                query_info: std::mem::transmute::<*mut c_void, QueryInfoFn>(ptrs[5]),
            })
        }
    }

    pub fn get_idle_seconds() -> Option<u64> {
        let fns = IDLE_FNS.get_or_init(load_fns).as_ref()?;

        unsafe {
            let display = (fns.open_display)(std::ptr::null());
            if display.is_null() {
                return None;
            }

            let root = (fns.default_root_window)(display);
            let info = (fns.alloc_info)();
            if info.is_null() {
                (fns.close_display)(display);
                return None;
            }

            let status = (fns.query_info)(display, root, info);
            let idle_ms = if status != 0 {
                Some((*info).idle as u64)
            } else {
                None
            };

            (fns.x_free)(info as *mut c_void);
            (fns.close_display)(display);

            idle_ms.map(|ms| ms / 1000)
        }
    }
}

// ---- macOS: CoreGraphics framework (always available) ----

#[cfg(target_os = "macos")]
mod platform {
    use std::os::raw::c_uint;

    type CGEventSourceStateID = c_uint;
    const CG_EVENT_SOURCE_STATE_COMBINED_SESSION: CGEventSourceStateID = 0;
    // kCGAnyInputEventType = ~0
    const CG_ANY_INPUT_EVENT_TYPE: c_uint = !0u32;

    #[link(name = "CoreGraphics", kind = "framework")]
    extern "C" {
        fn CGEventSourceSecondsSinceLastEventType(
            source_state: CGEventSourceStateID,
            event_type: c_uint,
        ) -> f64;
    }

    pub fn get_idle_seconds() -> Option<u64> {
        let seconds = unsafe {
            CGEventSourceSecondsSinceLastEventType(
                CG_EVENT_SOURCE_STATE_COMBINED_SESSION,
                CG_ANY_INPUT_EVENT_TYPE,
            )
        };
        if seconds >= 0.0 {
            Some(seconds as u64)
        } else {
            None
        }
    }
}

// ---- Windows: user32 GetLastInputInfo (always available) ----

#[cfg(target_os = "windows")]
mod platform {
    #[repr(C)]
    struct LastInputInfo {
        cb_size: u32,
        dw_time: u32,
    }

    extern "system" {
        fn GetLastInputInfo(plii: *mut LastInputInfo) -> i32;
        fn GetTickCount() -> u32;
    }

    pub fn get_idle_seconds() -> Option<u64> {
        unsafe {
            let mut lii = LastInputInfo {
                cb_size: std::mem::size_of::<LastInputInfo>() as u32,
                dw_time: 0,
            };
            if GetLastInputInfo(&mut lii) != 0 {
                let now = GetTickCount();
                let idle_ms = now.wrapping_sub(lii.dw_time) as u64;
                Some(idle_ms / 1000)
            } else {
                None
            }
        }
    }
}

// ---- Unsupported platforms ----

#[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
mod platform {
    pub fn get_idle_seconds() -> Option<u64> {
        None
    }
}
