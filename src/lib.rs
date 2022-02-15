#![deny(clippy::all)]

use napi::bindgen_prelude::Buffer;
use napi_derive::napi;

use std::ffi::{OsStr, OsString};
use std::os::windows::ffi::{OsStrExt, OsStringExt};

#[cfg(all(
    any(windows, unix),
    target_arch = "x86_64",
    not(target_env = "musl"),
    not(debug_assertions)
))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[napi]
pub fn plus_100(input: u32) -> u32 {
    input + 100
}

#[napi]
pub fn init() {
    unsafe {
        rust7z::init7z();
    }
}

#[napi]
pub fn open(path: String) -> u32 {
    unsafe {
        let k = u2w(path.as_str());
        rust7z::open(k.as_ptr()).file_count
    }
}

#[napi]
pub fn close() {
    unsafe {
        rust7z::close();
    }
}

#[napi]
pub fn file_name(i: u32) -> String {
    unsafe {
        let file = rust7z::getFileInfo(i);
        w2u(file.path)
    }
}

#[napi]
pub fn file_size(i: u32) -> u32 {
    unsafe {
        let file = rust7z::getFileInfo(i);
        file.size
    }
}

#[napi]
pub fn file_data(i: u32) -> Buffer {
    unsafe {
        let file = rust7z::getFileInfo(i);

        let v = vec![0; file.size as usize];
        rust7z::extractToBuf(v.as_ptr(), &i, 1);

        Buffer::from(v)
    }
}

fn u2w(u8str: &str) -> Vec<u16> {
    OsStr::new(u8str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<_>>()
}

fn w2u(wstr: *const u16) -> String {
    unsafe {
        let len = (0..std::isize::MAX)
            .position(|i| *wstr.offset(i) == 0)
            .unwrap();
        let slice = std::slice::from_raw_parts(wstr, len);
        OsString::from_wide(slice).to_string_lossy().into_owned()
    }
}
