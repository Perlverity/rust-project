use std::io;

fn main() {
    // const MAX_POINTS: u32 = 100_000;
    // let mut x = 5;
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);

    // let x = 5;
    // let x = x + 1;
    //
    // {
    //     let x = x * 2;
    //     println!("The value of x in the inner scope is {}", x)
    // }
    //
    // println!("The value of x is: {}", x);
    //
    // let spaces = "   ";
    // let spaces = spaces.len();
    //
    // println!("The value of spaces is: {}", spaces);
    //
    // let guess: u32 = "42".parse().expect("Not a number!");

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let element = a[index];

    println!("The value of the element at index {} is: {}", index, element);
}
