use std::ffi::{c_char, c_void, CString};

/// Implements stbi_write_png
///
/// # Panics
///
/// When the size of data is not equal to width * height * components
pub fn write_png(
    filename: &str,
    width: u32,
    height: u32,
    components: u32,
    data: &[u8],
) -> Result<(), ()> {
    assert!((width * height * components) as usize == data.len());

    let c_path = match CString::new(filename) {
        Err(_) => return Err(()),
        Ok(cstr) => cstr,
    };

    unsafe {
        match stbi_write_png(
            c_path.as_ptr(),
            width as i32,
            height as i32,
            components as i32,
            data.as_ptr() as *const c_void,
            (width * components) as i32,
        ) {
            0 => Err(()),
            _ => Ok(()),
        }
    }
}

extern "C" {
    fn stbi_write_png(
        filename: *const c_char,
        w: i32,
        h: i32,
        comp: i32,
        data: *const c_void,
        stride_in_bytes: i32,
    ) -> i32;
}
