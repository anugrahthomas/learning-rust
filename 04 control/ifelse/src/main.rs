fn main() {
    let a = 30;
    if a > 30 {
        println!("A is greater than 30");
    } else if a < 30 {
        println!("A is Smaller than 30");
    } else {
        println!("A is equal to 30");
    }

    // 'if' as an expression
    let b = if a == 30 { "a is 30" } else { "a is not 30" };
    println!("{b}\n");

    // loops infinite loop
    let mut x = 3;
    loop {
        if x == 0 {
            break;
        }
        println!("{}", x);
        x = x-1;
    }

    // while loop
    x = 3;
    while x <= 10 {
        println!("{}", x);
        x = x + 1;
    }

    // for loop
    for i in 1..10 {
        println!("{}", i);
    }

    // loop as an expression 
    let res = loop {
        let a = 30;
        if a == 30 {
            break 90;
        }
    };
    println!("res: {res}");

    // loop label

    let res = 'my_label: loop{
        let mut i = 10;
        loop{
            println!("{}", i);
            if i == 5 {
                break 'my_label 30 * i;
            }
            i = i - 1;
        }
    };
    println!("res: {}", res);
}
