#![allow(non_snake_case)]
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Arc;
use std::collections::HashMap;
use std::sync::RwLock;

static mut INDEX: Option<Arc<RwLock<HashMap<String, Vec<u8>>>>> = None;

#[no_mangle]
pub extern "C" fn fabric_init() -> bool {
    unsafe {
        if INDEX.is_none() {
            INDEX = Some(Arc::new(RwLock::new(HashMap::new())));
            true
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn fabric_index_data(key: *const c_char, data: *const u8, length: usize) -> bool {
    unsafe {
        if INDEX.is_none() { return false; }
        let key_cstr = CStr::from_ptr(key);
        let key_str = match key_cstr.to_str() {
            Ok(s) => s.to_string(),
            Err(_) => return false,
        };
        let data_slice = std::slice::from_raw_parts(data, length);
        if let Ok(mut index) = INDEX.as_ref().unwrap().write() {
            index.insert(key_str, data_slice.to_vec());
            true
        } else {
            false
        }
    }
}

#[no_mangle]
pub extern "C" fn fabric_search(query: *const c_char, result: *mut *mut c_char) -> bool {
    unsafe {
        if INDEX.is_none() { return false; }
        let query_cstr = CStr::from_ptr(query);
        let query_str = match query_cstr.to_str() {
            Ok(s) => s.to_lowercase(),
            Err(_) => return false,
        };
        
        if let Ok(index) = INDEX.as_ref().unwrap().read() {
            for (key, _) in index.iter() {
                if key.to_lowercase().contains(&query_str) {
                    let cstring = CString::new(key.clone()).unwrap();
                    *result = cstring.into_raw();
                    return true;
                }
            }
        }
        false
    }
}

#[no_mangle]
pub extern "C" fn fabric_free_string(s: *mut c_char) {
    unsafe {
        if !s.is_null() {
            CString::from_raw(s);
        }
    }
}
