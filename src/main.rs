#[cfg(target_os = "windows")]
windows::include_bindings!();

use Windows::Win32::Foundation::{HWND, LPARAM, LRESULT, PWSTR, WPARAM};
use Windows::Win32::Graphics::Gdi::{GetStockObject, WHITE_BRUSH};
use Windows::Win32::System::LibraryLoader::GetModuleHandleW;
use Windows::Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, DefWindowProcW, DispatchMessageW, GetMessageW, PostQuitMessage,
    RegisterClassExW, ShowWindow, TranslateMessage, CS_OWNDC, CW_USEDEFAULT, HCURSOR, HICON, HMENU,
    MSG, SW_SHOW, WM_CLOSE, WNDCLASSEXW, WS_EX_APPWINDOW, WS_OVERLAPPEDWINDOW,
};

extern "system" fn window_callback(
    hwnd: HWND,
    msg: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    match msg {
        WM_CLOSE => {
            unsafe { PostQuitMessage(0) };
            return LRESULT(0);
        }
        _ => {
            return unsafe { DefWindowProcW(hwnd, msg, wparam, lparam) };
        }
    }
}

fn into_pwstr(src: &str) -> (PWSTR, Vec<u16>) {
    let mut encoded: Vec<u16> = src.encode_utf16().chain([0u16]).collect();
    (PWSTR(encoded.as_mut_ptr()), encoded)
}

fn main() {
    let wcx = WNDCLASSEXW {
        cbSize: std::mem::size_of::<WNDCLASSEXW>() as u32,
        style: CS_OWNDC,
        lpfnWndProc: Some(window_callback),
        cbClsExtra: 0,
        cbWndExtra: 0,
        hInstance: unsafe { GetModuleHandleW(PWSTR::NULL) },
        hIcon: HICON::NULL,
        hCursor: HCURSOR::NULL,
        hbrBackground: unsafe { std::mem::transmute(GetStockObject(WHITE_BRUSH)) }, // Cast to HBRUSH!
        lpszMenuName: PWSTR::NULL,
        lpszClassName: into_pwstr("application_class").0,
        hIconSm: HICON::NULL,
    };

    unsafe { assert!(RegisterClassExW(&wcx) != 0) }

    let hwnd: HWND = unsafe {
        CreateWindowExW(
            WS_EX_APPWINDOW,
            wcx.lpszClassName,
            into_pwstr("Application").0,
            WS_OVERLAPPEDWINDOW,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            640,
            480,
            HWND::NULL,
            HMENU::NULL,
            wcx.hInstance,
            std::ptr::null_mut(),
        )
    };

    assert!(hwnd != HWND::NULL);

    unsafe { ShowWindow(hwnd, SW_SHOW) };

    unsafe {
        let mut msg = MSG::default();
        while GetMessageW(std::ptr::addr_of_mut!(msg), HWND::NULL, 0, 0).as_bool() != false {
            TranslateMessage(std::ptr::addr_of!(msg));
            DispatchMessageW(std::ptr::addr_of!(msg));
        }
    };
}
