use std::ffi::CString;
use std::os::raw::c_char;

#[link(name = "hello", kind = "static")]
extern "C" {
    fn hello();
}

extern "C" {
    fn c_add(a: i32, b: i32) -> i32;
}

extern "C" {
    fn puts(s: *const c_char);
    fn strlen(s: *const c_char) -> usize;
}

fn main() {
    unsafe {
        hello();
    }

    let a = 10;
    let b = 20;
    let ans = unsafe { c_add(a, b) };
    println!("ans is {}", ans);

    let s = "hello rust world.";
    let s_null_terminated = CString::new(s).unwrap();

    unsafe {
        puts(s_null_terminated.as_ptr());
    }

    let n = unsafe {
        strlen(s_null_terminated.as_ptr());
    };

    println!("s.len is {:?}", n);
}
