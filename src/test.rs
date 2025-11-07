#![allow(unused)]
use core::prelude::v1;
use std::fmt::format;
use std::sync::OnceLock;
use std::sync::{Arc, Mutex as StdMutex, MutexGuard};
use tokio::sync::OnceCell;

pub fn run() {
    test1();
}

//配置文件管理结构体
#[derive(Clone)]
struct FileManager {
    url: String,
    data: ini::Ini,
}
static SHARE_MANAGER: OnceLock<Arc<StdMutex<FileManager>>> = OnceLock::new();

impl FileManager {
    pub fn new() -> Self {
        //根据路劲读取ini文件内容
        let exe_dir = std::env::current_exe().expect("无法获取 exe 路径");
        let file_path = std::path::Path::new("config.ini").to_path_buf();
        let data = std::fs::read_to_string(&file_path).unwrap();
        let data = ini::Ini::load_from_str(&data).expect("无法解析 ini 文件");
        FileManager {
            url: file_path.to_str().unwrap().to_string(),
            data,
        }
    }
    pub fn get_url(&self) -> &str {
        &self.url
    }
    pub fn get_data(&self) -> &ini::Ini {
        &self.data
    }
    //写入信息
    pub fn write_data(&mut self, section: Option<&str>, key: &str, value: &str) {
        self.data.with_section(section).set(key, value);
        self.data.write_to_file(&self.url).expect("无法写入 ini 文件");
    }
    //获取节点信息
    pub fn read_data(&self, section: Option<&str>, key: &str) -> Option<String> {
        self.data.get_from(section, key).map(|v| v.to_string())
    }
}

fn test1() {
    let manger_mutex: &Arc<StdMutex<FileManager>> = SHARE_MANAGER.get_or_init(|| Arc::new(StdMutex::new(FileManager::new())));
    let mut v: Vec<std::thread::JoinHandle<()>> = vec![];
    for i in 1..=5 {
        let manger_mutex: Arc<StdMutex<FileManager>> = manger_mutex.clone();
        v.push(std::thread::spawn(move || {
            let mut manger: MutexGuard<'_, FileManager> = manger_mutex.lock().unwrap();
            let v1: Option<String> = manger.read_data(Some("app"), "version");
            print!("version:{:?}\t", v1);
            let version: String = format!("1.0.{}", i);
            manger.write_data(Some("app"), "version", &version);
            let v2: Option<String> = manger.read_data(Some("app"), "version");
            println!("Updated version:{:?}", v2);
        }));
    }
    for handle in v {
        handle.join().unwrap();
    }
    println!("Updated version:{:?}", SHARE_MANAGER.get().unwrap().lock().unwrap().read_data(Some("app"), "version"));
}
