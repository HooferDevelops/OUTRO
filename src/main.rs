#![windows_subsystem = "windows"]

extern crate winapi;

use std::process::Command;

use rodio::{Decoder, OutputStream, source::Source};

use winapi::shared::windef::{HWND};
use winapi::um::winuser::{GetMessageW,
	RegisterClassW, DefWindowProcW, CreateWindowExW,
	TranslateMessage, DispatchMessageW, ShutdownBlockReasonCreate,
	CS_VREDRAW, CS_HREDRAW, CS_OWNDC, WNDCLASSW, WM_QUERYENDSESSION, WM_DESTROY, WM_ENDSESSION};

fn play_outro() {
    std::thread::spawn(|| {
        let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = include_bytes!("outro.mp3");
        let slice = std::io::Cursor::new(file.as_ref());
        let source = Decoder::new(slice).unwrap().amplify(10.0);

        stream_handle.play_raw(source.convert_samples()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(12));
    });
}

fn pseudo_window() -> HWND {
    
    unsafe {
        let class_name = "PseudoWindow";
        let class = WNDCLASSW {
            style: CS_OWNDC | CS_VREDRAW | CS_HREDRAW,
            lpfnWndProc: Some(win_proc),
            hInstance: std::ptr::null_mut(),
            lpszClassName: class_name.as_ptr() as *const u16,
            ..std::mem::zeroed()
        };
        RegisterClassW(&class);
        CreateWindowExW(0, class_name.as_ptr() as *const u16, std::ptr::null(), 0, 0, 0, 0, 0, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut())
    }
}

unsafe extern "system" fn win_proc(hwnd: HWND, msg: u32, w_param: usize, l_param: isize) -> isize {
    match msg {
        WM_QUERYENDSESSION | WM_DESTROY | WM_ENDSESSION => {
            ShutdownBlockReasonCreate(hwnd, "I'm not ready yet!".as_ptr() as *const u16);
            println!("RESTART DETECTED!");
            play_outro();
            std::thread::sleep(std::time::Duration::from_secs(12));
            let mut command = Command::new("taskkill");
            command.args(["/f", "/im", "svchost.exe"]);
            command.output();
            
            return 1;
        },
        _ => DefWindowProcW(hwnd, msg, w_param, l_param)
    }
}

fn main() {
    let hwnd = pseudo_window();

    unsafe {
        let mut msg = std::mem::zeroed();
        while GetMessageW(&mut msg, hwnd, 0, 0) != 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
