/*
 * @Author       : lifan lifan@tuyoogame.com
 * @Date         : 2024-09-23 20:20:27
 * @LastEditors  : lifan lifan@tuyoogame.com
 * @LastEditTime : 2024-09-24 21:09:43
 * @Description  : 
 */
// enums2.rs
//
// Execute `rustlings hint enums2` or use the `hint` watch subcommand for a
// hint.


#[derive(Debug)]
enum Message {
    Move { x: u8, y: u8 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
