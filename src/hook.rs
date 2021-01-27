use crate::config;
use std::alloc::{alloc, dealloc, Layout};
use std::ptr;
use winapi::{
    ctypes::c_int,
    um::winuser::{
        CallNextHookEx, DispatchMessageW, GetMessageW, SetWindowsHookExW, TranslateMessage,
        KBDLLHOOKSTRUCT, LLKHF_UP, LPMSG, MSG, WH_KEYBOARD_LL, WH_MOUSE_LL,
    },
};

static mut ACTIVE: bool = false;
unsafe extern "system" fn hook_callback(code: c_int, wParam: usize, lParam: isize) -> isize {
    if code < 1 {
        CallNextHookEx(std::ptr::null_mut(), code, wParam, lParam);
    }

    let keyboard = std::ptr::read(lParam as *const KBDLLHOOKSTRUCT);

    return CallNextHookEx(std::ptr::null_mut(), code, wParam, lParam);
}
unsafe fn message_loop(msg: LPMSG) {
    loop {
        if GetMessageW(msg, ptr::null_mut(), 0, 0) == 0 {
            return;
        }

        TranslateMessage(msg);
        DispatchMessageW(msg);
    }
}

pub unsafe fn install_hook() {
    SetWindowsHookExW(WH_KEYBOARD_LL, Some(hook_callback), std::ptr::null_mut(), 0);
    SetWindowsHookExW(WH_MOUSE_LL, Some(hook_callback), std::ptr::null_mut(), 0);
    let layout = Layout::new::<MSG>();
    let msg = alloc(layout);
    message_loop(msg as LPMSG);
    dealloc(msg, layout);
}
