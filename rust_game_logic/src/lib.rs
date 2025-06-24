use std::ffi::{CString, CStr}; 
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn calculate_trajectory(
    angle: f32, 
    power: f32, 
    gravity: f32,
) -> *mut c_char {
    let time = (2.0 * power * angle.to_radians().sin()) / gravity;
    let distance = power * angle.to_radians().cos() * time;
    
    let result = format!(r#"{{"x": {}, "y": {}}}"#, distance, 0.0);
    CString::new(result).unwrap().into_raw() // 注意：调用者需负责释放内存
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(s: *mut c_char) {
    unsafe { CString::from_raw(s) }; // 显式标记 unsafe 块
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
