use libloading::{Library, Symbol};
use std::path::Path;
use tokio::time::{self, Duration};

use std::fs;
use json;
use console_subscriber;


mod utils;

use utils::get_current_timestamp;

async fn entry() -> Result<(), Box<dyn std::error::Error>>  {

    console_subscriber::init();

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

        // 定义 add 函数的类型
        let add: Symbol<unsafe extern "C" fn(i32, i32) -> i32> =
            lib.get(b"add").expect("Failed to load function add");

        // 调用 add 函数
        let result = add(5, 7);
        println!("Result of add(5, 7): {}", result);

        let proc: Symbol<unsafe extern "C" fn()> =
            lib.get(b"proc").expect("Failed to load function proc");

        // loop to create 5 tasks
        let mut tasks = vec![];
        let number_of_task = 1;
        for id in 0..number_of_task {
            let handle =tokio::spawn(periodic_task(id, *proc, period_ms));
            tasks.push(handle);
            println!("Task {} started", id);

        }

        for task in tasks {
            task.await.unwrap();
        }
    }
    Ok(())
}


fn main() {

    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(10) // 配置工作线程数量
        .enable_all()
        .build()
        .unwrap();

    runtime.block_on(async {
        // 调用你的主逻辑
        entry().await.unwrap();
    });
}


async fn periodic_task(id: u16, proc: unsafe extern "C" fn(), period_ms: u64) {
    let mut interval = time::interval(Duration::from_millis(period_ms)); 
    let mut last_execution = get_current_timestamp();

    interval.tick().await;

    loop {
        interval.tick().await;

        // ------------ Task Logic ------------ 
        unsafe {
            proc();
        }

        // added some workload here 
        // tokio::time::sleep(Duration::from_millis(1)).await; 

        // ------------ Task Logic End ------------ 

        let timestamp_ms = get_current_timestamp();
        let delta_ms = timestamp_ms - last_execution;
        last_execution = timestamp_ms;
    
        if delta_ms as f64 >= ((period_ms as f64) * 1.1 ) || delta_ms as f64 <= ((period_ms as f64) * 0.9) {
            println!("TaskID: {} is running at {}, every {} ms!", id, timestamp_ms, delta_ms);
        }
    }
}
