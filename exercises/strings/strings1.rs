/*
 * @Author       : lifan lifan@tuyoogame.com
 * @Date         : 2024-09-23 20:20:27
 * @LastEditors  : lifan lifan@tuyoogame.com
 * @LastEditTime : 2024-09-24 22:05:31
 * @Description  : 
 */
// strings1.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings1` or use the `hint` watch subcommand for a
// hint.

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    String::from("blue")
}
