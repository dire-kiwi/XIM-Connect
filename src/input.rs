use crate::window;
use std::alloc::{alloc, dealloc, Layout};
use std::convert::TryFrom;
use std::mem;
use std::ptr;
use std::sync::Arc;
use winapi::{
    shared::{
        hidusage::{HID_USAGE_GENERIC_KEYBOARD, HID_USAGE_GENERIC_MOUSE, HID_USAGE_PAGE_GENERIC},
        minwindef::{DWORD, HINSTANCE, LPARAM, LPVOID, LRESULT, PUINT, UINT, WPARAM},
        windef::HWND,
    },
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            ChangeWindowMessageFilterEx, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GetMessageW, GetRawInputData, GetWindowLongPtrW, RegisterClassExW,
            RegisterRawInputDevices, RegisterWindowMessageW, SetWindowLongPtrW, ShowCursor,
            ShowWindow, TranslateMessage, GWLP_USERDATA, HRAWINPUT, LPMSG, MOUSE_MOVE_RELATIVE,
            MSG, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_CAPTUREMOUSE, RIDEV_INPUTSINK,
            RIDEV_NOLEGACY, RID_INPUT, RIM_TYPEMOUSE, RI_MOUSE_WHEEL, WM_INPUT, WM_KEYDOWN,
            WM_KEYUP, WM_QUERYENDSESSION, WNDCLASSEXW,
        },
    },
};

use crate::manager::ManagerEvent;
use crate::xim;
use std::mem::transmute_copy;
use std::sync::mpsc::Sender;
lazy_static! {
    static ref CB_SIZE_HEADER: UINT = mem::size_of::<RAWINPUTHEADER>() as UINT;
}

pub unsafe fn proc_raw_input(hWnd: HWND, l_param: LPARAM) -> bool {
    let mut pcb_size = 0;
    let tx =
        std::ptr::read(GetWindowLongPtrW(hWnd, GWLP_USERDATA) as *const Arc<window::WindowData>)
            .clone();

    let is_mouse_move_relative = |ri: RAWINPUT| {
        ri.header.dwType == RIM_TYPEMOUSE && ri.data.mouse().usFlags == MOUSE_MOVE_RELATIVE
    };

    let get_raw_input_data = |data: LPVOID, size: PUINT| {
        GetRawInputData(l_param as HRAWINPUT, RID_INPUT, data, size, *CB_SIZE_HEADER)
    };

    if get_raw_input_data(ptr::null_mut(), &mut pcb_size) == 0 {
        let layout = Layout::from_size_align(pcb_size as usize, 1).unwrap();
        let data = alloc(layout);
        let mut res = false;

        if get_raw_input_data(data as LPVOID, &mut pcb_size) == pcb_size {
            let ri = std::ptr::read(data as *const RAWINPUT);
            let keyboard = ri.data.keyboard();
            let msg = keyboard.Flags;
            let pressed = match msg {
                0 => xim::ButtonEvent::ButtonPressed,
                2 => xim::ButtonEvent::ButtonPressed,
                _ => xim::ButtonEvent::ButtonReleased,
            };
            let key = keyboard.VKey as i32;

            if pressed != xim::ButtonEvent::NOOP {
                tx.xim_tx.send(xim::XIMEvent::KeyboardButton(
                    u16::try_from(key).unwrap(),
                    pressed,
                ));
            }

            let mouse = ri.data.mouse();

            tx.xim_tx
                .send(xim::XIMEvent::MouseButtonEvent(mouse.usButtonFlags));
            if is_mouse_move_relative(ri) {
                let wheel = if mouse.usButtonFlags & RI_MOUSE_WHEEL == RI_MOUSE_WHEEL {
                    ((transmute_copy::<u16, i16>(&mouse.usButtonData) as f32) / 120f32) as i16
                } else {
                    0
                };

                let x = i16::try_from(mouse.lLastX).unwrap_or(0);
                let y = i16::try_from(mouse.lLastY).unwrap_or(0);

                tx.xim_tx.send(xim::XIMEvent::MouseMoveEvent {
                    x: x,
                    y: y,
                    wheel: wheel,
                });
                res = true;
            }
        }

        dealloc(data, layout);
        return res;
    }

    false
}
