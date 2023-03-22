fn main() {
    // Raw pointers
    let mut num = 5;

    let ptr1 = &num as *const i32;
    let ptr2 = &mut num as *mut i32;

    // Try to uncomment this. Won't work.
    // let try_deref = *ptr1;

    let address = 0x012345usize;
    let _rng_ptr = address as *const i32;
    // Undefined behaviour if we deref address
    // because it points to a random memory location.

    unsafe {
        println!("ptr1 is: {}", *ptr1);
        println!("ptr2 is: {}", *ptr2);
    }

    // Don't do this in a real programm!
    // unsafe {
    //     println!("random pointer (dereferenced) is: {:?}", *_rng_ptr);
    // }
    // error: process didn't exit successfully:
    // `C:\Users\...` (exit code: 0xc0000005, STATUS_ACCESS_VIOLATION)

    // Unsafe functions

    unsafe fn dangerous() {}

    // Won't compile.
    // dangerous();

    unsafe {
        dangerous();
    }

    // Safe abstractions:
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // split_at_mut uses unsafe {}.
    use std::slice;

    fn _split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    // Call external code.
    extern "C" {
        fn abs(input: i32) -> i32;
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // In the following example, we make the call_from_c function accessible from C code,
    // after it’s compiled to a shared library and linked from C:
    #[no_mangle]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    // This usage of extern does not require unsafe.

    // Accessing or Modifying a Mutable Static Variable.
    static HELLO_WORLD_INSIDE_MAIN: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD_OUTSIDE_MAIN);
    println!("name is: {}", HELLO_WORLD_INSIDE_MAIN);

    // Mutate static variable. (unsafe)
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    // Read from a mutable static variable. (unsafe)
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }

    // Unsafe trait. (One of the methods has an unverifiable variant)
    unsafe trait Foo {
        // methods go here
    }
    
    unsafe impl Foo for i32 {
        // method implementations go here
    }
    // As an example, recall the Sync and Send marker traits we discussed 
    // in the “Extensible Concurrency with the Sync and Send Traits” section 
    // in Chapter 16: the compiler implements these traits automatically if 
    // our types are composed entirely of Send and Sync types. If we implement 
    // a type that contains a type that is not Send or Sync, such as raw pointers, 
    // and we want to mark that type as Send or Sync, we must use unsafe. Rust can’t 
    // verify that our type upholds the guarantees that it can be safely sent across 
    // threads or accessed from multiple threads; therefore, we need to do those 
    // checks manually and indicate as such with unsafe.

    // Accessing unions is unsafe. (Read documentation, mainly to interact with C unions)
    union MyUnion {
        f1: u32,
        f2: f32,
    } // Size of unions is dictates by its largest member.
    // Accessing union fields is unsafe because Rust can’t guarantee 
    // the type of the data currently being stored in the union instance. 

    let u = MyUnion { f1: 1 };
    let f = unsafe { u.f1 }; // Unsafe.
    println!("{}", f);
    
}

// Static global variable.
static HELLO_WORLD_OUTSIDE_MAIN: &str = "Hello, world!";
// Lets mutate this.
static mut COUNTER: u32 = 0;
