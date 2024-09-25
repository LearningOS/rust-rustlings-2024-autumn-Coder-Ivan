/*
 * @Author       : lifan lifan@tuyoogame.com
 * @Date         : 2024-09-23 20:20:27
 * @LastEditors  : lifan lifan@tuyoogame.com
 * @LastEditTime : 2024-09-24 22:07:09
 * @Description  : 
 */
// strings2.rs
//
// Make me compile without changing the function signature!
//
// Execute `rustlings hint strings2` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let word = String::from("green"); // Try not changing this line :)
    if is_a_color_word(&word) {
        println!("That is a color word I know!");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}
