use windows::{
    Win32::Foundation::{BOOL, HWND, LPARAM, PWSTR},
    Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW},
};

fn main() -> windows::core::Result<()> {
    unsafe { EnumWindows(Some(enum_window), 0).ok() }
}

extern "system" fn enum_window(window: HWND, _: LPARAM) -> BOOL {
    unsafe {
        let mut text: [u16; 512] = [0; 512];
        let len = GetWindowTextW(window, PWSTR(text.as_mut_ptr()), text.len() as i32);
        let text = String::from_utf16_lossy(&text[..len as usize]);

        if !text.is_empty() {
            println!("{}", text);
        }

        return true.into();
    }
}
