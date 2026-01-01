fn main() {
    // Mutable Variables
    let mut a = 43;
    println!("a: {a}");
    a = 42;
    println!("a: {a}");

    // Imutable Variables
    let b = 10; // variable with let is in lowercase, we can use UPPERCASE but not standard
    const PI: f32 = 3.14; // vaiable with const is in UPPERCASE, we can use lowercase but it is not upto standard
    println!("B: {b}");
    println!("pi: {PI}");

    // readability of a number
    let c = 234u8; // we can suffix the type
    let d = 1_00_00_000; // _ is ignored but more readable
    println!("c: {}, d: {}", c, d);


    // boolean
    let t = true;
    let f = false;
    println!("t: {t}, f: {f}");

    // character
    let ch = 'a';
    println!("ch: {ch}");
    
    // tuple (compound datatype) holds different datatype
    let tup = ('a', 300, 3.09);
    let (x, y, z) = tup; // destructuring of tup
    println!("x: {x}, y: {y}, z: {z}");
    println!("tup at 0: {}", tup.0); // also access in this way

    // Array // same type
    let arr = [1,2,3,4,5];
    let arr1 = [2; 5]; // similar to [2,2,2,2,2]
    println!("arr: {}", arr[0]);

}
