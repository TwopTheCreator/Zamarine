use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_ulong};
use std::ptr;

#[repr(C)]
pub struct DriveInfo {
    name: [u8; 260], // MAX_PATH
    pub type_: c_ulong,
}

#[link(name = "localwin32")]
extern "C" {
    fn get_drives(drives: *mut DriveInfo, max_drives: c_int) -> c_int;
    fn read_file(path: *const c_char, buffer: *mut *mut c_char, size: *mut c_ulong) -> bool;
    fn write_file(path: *const c_char, data: *const c_char, size: c_ulong) -> bool;
    fn find_window_by_title(title: *const c_char) -> *mut std::os::raw::c_void;
    fn close_window(hwnd: *mut std::os::raw::c_void) -> bool;
    fn bring_window_to_front(hwnd: *mut std::os::raw::c_void) -> bool;
}

pub fn list_drives() -> Vec<(String, u32)> {
    let mut drives: [DriveInfo; 26] = unsafe { std::mem::zeroed() };
    let count = unsafe { get_drives(drives.as_mut_ptr(), 26) };
    let mut result = Vec::new();
    for i in 0..count as usize {
        let name = unsafe { CStr::from_ptr(drives[i].name.as_ptr() as *const c_char) }
            .to_string_lossy()
            .into_owned();
        result.push((name, drives[i].type_ as u32));
    }
    result
}

pub fn read_file_rust(path: &str) -> Option<Vec<u8>> {
    let c_path = CString::new(path).unwrap();
    let mut buffer: *mut c_char = ptr::null_mut();
    let mut size: c_ulong = 0;
    let success = unsafe { read_file(c_path.as_ptr(), &mut buffer, &mut size) };
    if success {
        let slice = unsafe { std::slice::from_raw_parts(buffer as *const u8, size as usize) };
        let data = slice.to_vec();
        unsafe { libc::free(buffer as *mut libc::c_void) };
        Some(data)
    } else {
        None
    }
}

pub fn write_file_rust(path: &str, data: &[u8]) -> bool {
    let c_path = CString::new(path).unwrap();
    unsafe { write_file(c_path.as_ptr(), data.as_ptr() as *const c_char, data.len() as c_ulong) }
}

pub struct Window {
    hwnd: *mut std::os::raw::c_void,
}

impl Window {
    pub fn find(title: &str) -> Option<Self> {
        let c_title = CString::new(title).unwrap();
        let hwnd = unsafe { find_window_by_title(c_title.as_ptr()) };
        if hwnd.is_null() {
            None
        } else {
            Some(Window { hwnd })
        }
    }

    pub fn close(&self) -> bool {
        unsafe { close_window(self.hwnd) }
    }

    pub fn bring_to_front(&self) -> bool {
        unsafe { bring_window_to_front(self.hwnd) }
    }
}
