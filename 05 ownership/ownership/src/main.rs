use std::io;

fn main() {
    // memory inside stack
    let a = 43; // store in stack
    println!("a: {a}");

    // heap memory
    let mut inp = String::new(); // inp in stack pointing to heap memory
    io::stdin().read_line(&mut inp).expect("not able to read");
    let inp = inp.trim();
    println!("inp: {inp}");

    // ownership
    {
        let x = "hello"; // scope starts
        println!("{x}");
    } // scope ends

    let s1 = String::from("hello");

    let mut s2 = s1.clone(); // deep copy

    s2.push_str("world");
    println!("s1: {s1}, s2: {s2}"); // s1: hello, s2: helloworld

}
