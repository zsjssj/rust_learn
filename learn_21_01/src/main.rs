//参考 https://kaisery.github.io/trpl-zh-cn/ch21-01-single-threaded.html
use std::{
    fs,
    io::{BufReader, prelude::*},
    net::{TcpListener, TcpStream},
};

fn main() {
    //在地址 127.0.0.1:7878 上监听传入的 TCP 流
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("Connection established!"); //当获取到传入的流，它会打印一条消息
        handle_connection(stream);
    }
}

// fn handle_connection(mut stream: TcpStream) {
//     //1.读取请求
//     let buf_reader = BufReader::new(&stream);
//     let http_request: Vec<_> = buf_reader.lines().map(|result| result.unwrap()).take_while(|line| !line.is_empty()).collect();
//     println!("\x1b[32mRequest: {http_request:#?}\x1b[0m");
//     //2.编写响应
//     let status_line = "HTTP/1.1 200 OK";
//     //3.返回真正的 HTML 文件内容
//     let contents = fs::read_to_string("assets/hello.html").unwrap();
//     let length = contents.len();
//     let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//     stream.write_all(response.as_bytes()).unwrap();
// }

// //4.验证请求并有选择的进行响应
// fn handle_connection(mut stream: TcpStream) {
//     let buf_reader = BufReader::new(&stream);
//     let request_line = buf_reader.lines().next().unwrap().unwrap();
//     if request_line == "GET / HTTP/1.1" {
//         let status_line = "HTTP/1.1 200 OK";
//         let contents = fs::read_to_string("assets/hello.html").unwrap();
//         let length = contents.len();
//         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//         stream.write_all(response.as_bytes()).unwrap();
//     } else {
//         let status_line = "HTTP/1.1 404 NOT FOUND";
//         let contents = fs::read_to_string("assets/404.html").unwrap();
//         let length = contents.len();
//         let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
//         stream.write_all(response.as_bytes()).unwrap();
//     }
// }

//5.稍加重构
fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    let (status_line, filename) = if request_line == "GET / HTTP/1.1" {
        ("HTTP/1.1 200 OK", "assets/hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "assets/404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}
