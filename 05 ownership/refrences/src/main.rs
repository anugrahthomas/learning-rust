fn main() {
    // no refrence
    let str = String::from("This is String");
    let (str, len) = no_ref(str); // the function returns the tuple
    println!("String: {str}, length: {len}");

    // using refrences
    let length = y_ref(&str);
    println!("length of string: {length}");
}

fn no_ref(s: String) -> (String, usize) {
    // in pure ownserhip we have to return the owner if further used inside the main/other functional scope, bcz main send the ownership to this so after that instruction the ownership sent to noRef therefore futher cant use
    let len = s.len();
    (s, len)
}

fn y_ref(s: &String) -> usize {
    // here the owner of s is still main, therefore no ownership has change, here only we borrow the s string for use, so we cannot mutate borrowed string, we can only use it for further operations
    let len = s.len();
    len
}

// if we want to mutate the string s inside this we can give mutable refrence y_ref(&mut str);
