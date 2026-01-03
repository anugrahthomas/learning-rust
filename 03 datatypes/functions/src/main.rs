use std::io;
fn main() {
    let mut s: String = Default::default();
    println!("Enter name");
    io::stdin().read_line(&mut s).expect("Error while reading");
    let s = s.trim();
    greet(s);

    print();

    let sum: i32 = add(10, 30);
    println!("sum: {}", sum);

    
    let y: i32 = {
        let x: i32 = 3;
        x
    };
    println!("y: {y}");
}

fn print() {
    println!("Hello World");
}

fn greet(name: &str) {
    println!("Hello {name}");
}

fn add(x: i32, y: i32) -> i32 {
    x + y // or use return x+y;
}
