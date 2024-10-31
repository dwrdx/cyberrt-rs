use libloading::{Library, Symbol};
use std::path::Path;
use tokio::time::{self, Duration};

use std::fs;
use json;

mod utils;

use utils::get_current_timestamp;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>  {

    let json_content = fs::read_to_string("test.json")
        .expect("Failed to read JSON file");

    // 解析 JSON 内容
    let parsed = json::parse(&json_content)?;

    // 访问 JSON 中的字段
    let name = parsed["name"].as_str().unwrap_or("Unknown");
    let period_ms = parsed["period_ms"].as_u64().unwrap_or(0);



    // 动态加载.so文件
    let lib_path = Path::new("/Users/exu/Workarea/cyberrt-rs/tests/libmylib.so");
    
    // 通过 libloading 加载 .so 文件

    unsafe {
        let lib = Library::new(lib_path).expect("Failed to load the shared library");

        // 定义 hello_from_c 函数的类型
        let hello_from_c: Symbol<unsafe extern "C" fn()> =
            lib.get(b"hello_from_c").expect("Failed to load function hello_from_c");

        // 定义 add 函数的类型
        let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            lib.get(b"add").expect("Failed to load function add");

        // 调用 add 函数
        let result = add(5, 7);
        println!("Result of add(5, 7): {}", result);

        let proc: Symbol<unsafe extern "C" fn()> =
            lib.get(b"proc").expect("Failed to load function proc");

        // 调用 hello_from_c 函数
        // loop to create 5 tasks
        let mut tasks = vec![];
        for id in 0..1000 {
            let handle =tokio::spawn(periodic_task(id, *hello_from_c, period_ms));
            tasks.push(handle);
            println!("Task {} started", id);

        }

        for task in tasks {
            task.await.unwrap();
        }

        proc();


    }
    Ok(())
}




async fn periodic_task(id: u16, proc: unsafe extern "C" fn(), period_ms: u64) {
    let mut interval = time::interval(Duration::from_millis(period_ms)); // 每5秒触发一次
    let mut last_execution = get_current_timestamp();

    loop {
        interval.tick().await;
        // 这里放置要周期性执行的代码
        unsafe {
            proc();
        }

        let timestamp_ms = get_current_timestamp();
        let delta_ms = timestamp_ms - last_execution;
        last_execution = timestamp_ms;
    
        if delta_ms >= ((period_ms + 10u64) as u128) || delta_ms <= ((period_ms - 10u64) as u128) {
            println!("[TaskID:{}] is running at {}, every {} ms!", id, timestamp_ms, delta_ms);
        }
    }
}
