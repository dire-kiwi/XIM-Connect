use crate::xim;
use winapi::{
    shared::{
        hidusage::{HID_USAGE_GENERIC_MOUSE,HID_USAGE_GENERIC_KEYBOARD,HID_USAGE_PAGE_GENERIC},
        minwindef::{DWORD, HINSTANCE, LPARAM, LPVOID, LRESULT, PUINT, UINT, WPARAM},
        windef::{HWND, RECT, POINT},
    },
    um::{
        libloaderapi::GetModuleHandleW,
        winuser::{
            ChangeWindowMessageFilterEx, CreateWindowExW, DefWindowProcW, DispatchMessageW,
            GetMessageW, GetRawInputData, GetWindowLongPtrW, RegisterClassExW, 
            RegisterRawInputDevices, RegisterWindowMessageW, SetWindowLongPtrW, TranslateMessage,
            GWLP_USERDATA, HRAWINPUT, LPMSG, MOUSE_MOVE_RELATIVE, MSG, RAWINPUT, RAWINPUTDEVICE, GetClientRect, ClientToScreen,SetRect,ClipCursor,
            RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT, RIM_TYPEMOUSE, WM_INPUT, ShowWindow, SetForegroundWindow,LockSetForegroundWindow,
            WM_QUERYENDSESSION, WNDCLASSEXW, RI_MOUSE_WHEEL, WS_OVERLAPPEDWINDOW, CW_USEDEFAULT, WS_VISIBLE, ShowCursor, SetLayeredWindowAttributes,
           WS_EX_COMPOSITED , WS_EX_LAYERED , WS_EX_NOACTIVATE , WS_EX_TOPMOST , WS_EX_TRANSPARENT,WS_EX_TOOLWINDOW, RegisterHotKey, VK_PAUSE, WM_HOTKEY
        },
    },
};

use std::sync::mpsc::Sender;
use widestring::U16CString;
use std::mem; 
use std::ptr; 
use std::sync::Arc;
use std::alloc::{alloc, dealloc, Layout};
use crate::input::proc_raw_input;
const MSGFLT_ALLOW: DWORD = 1;
use crate::register::{register_devices, unregister_devices};
use crate::manager::{ManagerEvent};

lazy_static! {
    static ref WM_TASKBAR_CREATED: UINT =
        unsafe { RegisterWindowMessageW(U16CString::from_str("TaskbarCreated").unwrap().as_ptr()) };
    static ref CB_SIZE_HEADER: UINT = mem::size_of::<RAWINPUTHEADER>() as UINT;
    static ref CLASS_NAME: U16CString = U16CString::from_str("W10Wheel/R_WM").unwrap();
    static ref WINDOW_NAME: U16CString = U16CString::from_str("W10dWheel/R_WM").unwrap();
   
}
static mut HOTKEY_ACTIVE: bool = false;
fn make_window_class(h_instance: HINSTANCE) -> WNDCLASSEXW {
    WNDCLASSEXW {
        cbSize: (mem::size_of::<WNDCLASSEXW>()) as UINT,
        cbClsExtra: 0,
        cbWndExtra: 0,
        hbrBackground: ptr::null_mut(),
        hCursor: ptr::null_mut(),
        hIcon: ptr::null_mut(),
        hIconSm: ptr::null_mut(),
        hInstance: h_instance,
        lpfnWndProc: Some(window_proc),
        lpszClassName: CLASS_NAME.as_ptr(),
        lpszMenuName: ptr::null_mut(),
        style: 0,
    }
}

unsafe extern "system" fn window_proc(
    hwnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {
 
    match msg {
        WM_INPUT => {
            if HOTKEY_ACTIVE && proc_raw_input(hwnd, l_param) {
                return 0;
            }
        }
        WM_QUERYENDSESSION => {
            return 0;
        }
        WM_HOTKEY => {
            let xim_manager = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut xim::XIMManager;
  
            if w_param == 1 {
                HOTKEY_ACTIVE = !HOTKEY_ACTIVE;
                if HOTKEY_ACTIVE {
                
                    match (*xim_manager).connect() {
                        Ok(_) => println!("connected"),
                        Err(e) => println!("not connected {:?}",e)
                    }
                    register_devices(hwnd);
                    ShowWindow(hwnd, 5);
                    SetForegroundWindow(hwnd);
                    let mut rc: RECT = RECT { top:0,left:0,right:0,bottom:0};
                    println!("here");
                    GetClientRect(hwnd, &mut rc);
                    println!("{} {} {} {}", rc.left, rc.top, rc.right, rc.bottom  );
                    let mut pt = POINT { x: rc.left, y: rc.top };
                    let mut pt2 = POINT { x: rc.right, y: rc.bottom };
                    ClientToScreen(hwnd, &mut pt);
                    ClientToScreen(hwnd, &mut pt2);
                    SetRect(&mut rc, pt.x, pt.y, pt2.x, pt2.y);

                    // Confine the cursor.
                    ClipCursor(&mut rc);
                } else {
                    ShowWindow(hwnd, 0);
                    unregister_devices();
                }
               
                
            }
            return 0;
        } 
        
        _ => {
          
        }, 
        
    };

    DefWindowProcW(hwnd, msg, w_param, l_param)
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

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Clone)]
pub struct WindowData {
    pub xim_tx: Sender<xim::XIMEvent>,
    pub manager_tx: Sender<ManagerEvent>
}

pub fn process_events(xim_tx: Sender<xim::XIMEvent>, manager_tx: Sender<ManagerEvent>) {
    unsafe {
        
        let h_instance = GetModuleHandleW(ptr::null());
        let window_class = make_window_class(h_instance);
        if RegisterClassExW(&window_class) != 0 {
            let hwnd = CreateWindowExW(
                WS_EX_LAYERED |WS_EX_TOOLWINDOW ,
                CLASS_NAME.as_ptr(),
                WINDOW_NAME.as_ptr(),
                WS_OVERLAPPEDWINDOW,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                ptr::null_mut(),
                ptr::null_mut(),
                h_instance,
                ptr::null_mut(),
            );
            let userdata = Arc::new(WindowData { xim_tx: xim_tx, manager_tx: manager_tx });
            let mtx = userdata.clone();
            mtx.manager_tx.send(ManagerEvent::WindowCreated(&hwnd as *const HWND as i32));
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, &userdata as *const Arc<WindowData> as i32);
            ChangeWindowMessageFilterEx(hwnd, *WM_TASKBAR_CREATED, MSGFLT_ALLOW, ptr::null_mut());
            RegisterHotKey(hwnd, 1, 0, VK_PAUSE as u32);
        
            let layout = Layout::new::<MSG>();
            let msg = alloc(layout);
            message_loop(msg as LPMSG);
            dealloc(msg, layout);
        }
    }
}
