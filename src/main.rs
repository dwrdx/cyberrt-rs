use libloading::{Library, Symbol};
use std::path::Path;

fn main() {
    // 动态加载.so文件
    let lib_path = Path::new("/Users/exu/Workarea/cyberrt-rs/tests/libmylib.so");
    
    // 通过 libloading 加载 .so 文件

    unsafe {
        let lib = Library::new(lib_path).expect("Failed to load the shared library");

        // 定义 hello_from_c 函数的类型
        let hello_from_c: Symbol<unsafe extern "C" fn()> =
            lib.get(b"hello_from_c").expect("Failed to load function hello_from_c");

        // 调用 hello_from_c 函数
        hello_from_c();

        // 定义 add 函数的类型
        let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            lib.get(b"add").expect("Failed to load function add");

        // 调用 add 函数
        let result = add(5, 7);
        println!("Result of add(5, 7): {}", result);
    }
}
