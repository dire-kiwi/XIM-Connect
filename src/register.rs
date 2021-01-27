use winapi::{
  shared::{
        hidusage::{HID_USAGE_GENERIC_MOUSE,HID_USAGE_GENERIC_KEYBOARD,HID_USAGE_PAGE_GENERIC},
        minwindef::{DWORD, HINSTANCE, LPARAM, LPVOID, LRESULT, PUINT, UINT, WPARAM},
        windef::HWND,
    },   
    um::winuser::{RAWINPUTDEVICE, RIDEV_INPUTSINK, RegisterRawInputDevices, RIDEV_CAPTUREMOUSE, RIDEV_NOLEGACY , RIDEV_REMOVE}
};

use std::mem;

fn make_raw_input_devices(hwnd: HWND) -> Vec<RAWINPUTDEVICE> {
    let mouse =  RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: RIDEV_INPUTSINK | RIDEV_NOLEGACY | RIDEV_CAPTUREMOUSE,
        hwndTarget: hwnd,
    };

    let keyboard = RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_KEYBOARD,
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: hwnd,
    };
    vec![mouse, keyboard]
}
fn make_remove_raw_input_devices() -> Vec<RAWINPUTDEVICE> {
    let mouse =  RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_MOUSE,
        dwFlags: RIDEV_REMOVE,
        hwndTarget: std::ptr::null_mut(),
    };

    let keyboard = RAWINPUTDEVICE {
        usUsagePage: HID_USAGE_PAGE_GENERIC,
        usUsage: HID_USAGE_GENERIC_KEYBOARD,
        dwFlags: RIDEV_REMOVE,
        hwndTarget:  std::ptr::null_mut(),
    };
    vec![mouse, keyboard]
}
pub unsafe fn register_devices(hWnd: HWND) {
    let mut rid_array = make_raw_input_devices(hWnd);
    RegisterRawInputDevices(
        rid_array.as_mut_ptr(),
        rid_array.len() as UINT,
        mem::size_of::<RAWINPUTDEVICE>() as UINT,
    );
}

pub unsafe fn unregister_devices() {
    let mut rid_array = make_remove_raw_input_devices();
    RegisterRawInputDevices(
        rid_array.as_mut_ptr(),
        rid_array.len() as UINT,
        mem::size_of::<RAWINPUTDEVICE>() as UINT,
    );
}