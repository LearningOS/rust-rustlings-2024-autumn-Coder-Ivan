/*
 * @Author       : lifan lifan@tuyoogame.com
 * @Date         : 2024-09-23 20:20:27
 * @LastEditors  : lifan lifan@tuyoogame.com
 * @LastEditTime : 2024-09-24 20:49:47
 * @Description  : 
 */
// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
