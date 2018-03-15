extern crate strsim;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::os::raw::c_double;

use strsim::jaro_winkler as rs_jaro_winkler;

#[no_mangle]
pub extern "C" fn jaro_winkler(s_ptr_a: *const c_char, s_ptr_b: *const c_char) -> c_double {
  let (c_str_a, c_str_b);
  unsafe {
    c_str_a = CStr::from_ptr(s_ptr_a);
    c_str_b = CStr::from_ptr(s_ptr_b);
  }

  // Panic if the strings are no good.
  let str_a = c_str_a.to_str().unwrap();
  let str_b = c_str_b.to_str().unwrap();
  
  rs_jaro_winkler(str_a, str_b)
}
