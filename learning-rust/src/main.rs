use std::{thread, time};

fn main() {
    let line1 , line4 = ["_" ; 5];
    loop {
        thread::sleep(time::Duration::from_millis(200));
        print!("{}[2J", 27 as char);
        println!(line1);
        println!(line2);
        println!(line3);
        println!(line4);
    }
}
