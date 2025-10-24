mod addin;

use std::{
    ffi::{c_int, c_long, c_void},
    sync::atomic::{AtomicI32, Ordering},
};

use addin1c::{AttachType, create_component, cstr1c, destroy_component};

pub static PLATFORM_CAPABILITIES: AtomicI32 = AtomicI32::new(-1);

/// # Safety
///
/// Component must be non-null.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn GetClassObject(name: *const u16, component: *mut *mut c_void) -> c_long {
    unsafe {
        match *name as u8 {
            b'1' => {
                let addin = addin::Addin::new();
                create_component(component, addin)
            }
            _ => 0,
        }
    }
}

/// # Safety
///
/// Component must be returned from `GetClassObject`, the function must be called once for each component.
#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DestroyObject(component: *mut *mut c_void) -> c_long {
    unsafe { destroy_component(component) }
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn GetClassNames() -> *const u16 {
    cstr1c!("1").as_ptr()
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn SetPlatformCapabilities(capabilities: c_int) -> c_int {
    PLATFORM_CAPABILITIES.store(capabilities, Ordering::Relaxed);
    3
}

#[allow(non_snake_case)]
#[unsafe(no_mangle)]
pub extern "C" fn GetAttachType() -> AttachType {
    AttachType::Any
}
