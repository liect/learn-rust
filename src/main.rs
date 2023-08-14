fn main() {
    println!("Hello, world!");
    let a:u8 = 255;
    let b = a.saturating_add(255);
    println!("a = {}", a);
    println!("b = {}", b);
    for i in 1..=5 {
        println!("i = {}", i);
    }
    for i in 'a'..='z' {
        println!("i = {}", i);
        
    }
}
