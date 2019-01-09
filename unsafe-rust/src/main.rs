use std::slice;

unsafe fn dangerous() {}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid),
        )
    }
}

extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world";
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe trait Foo {}
unsafe impl Foo for i32 {}

fn main() {
    {
        let mut num = 5;
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;

        unsafe {
            println!("r1 is: {}", *r1);
            println!("r2 is: {}", *r2);
        }
    }

    {
        let address = 0x012345usize;
        let r = address as *const i32;

        // let slice = unsafe { slice::from_raw_parts_mut(r, 10000) };
    }

    {
        unsafe {
            dangerous();
        }
    }

    {
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let r = &mut v[..];
        // let (a, b) = r.split_at_mut(3);
        let (a, b) = split_at_mut(r, 3);

        assert_eq!(a, &mut [1, 2, 3]);
        assert_eq!(b, &mut [4, 5, 6]);
    }

    {
        unsafe {
            println!("Absolute value -f -3 according to C: {:?}", abs(-3));
        }
    }

    {
        println!("name is {}", HELLO_WORLD);
    }

    {
        add_to_count(3);

        unsafe {
            println!("COUNTER: {}", COUNTER);
        }
    }
}
