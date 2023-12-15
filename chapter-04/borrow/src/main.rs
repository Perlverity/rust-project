fn main() {
    let mut s = String::from("hello");

    s.push_str(", world!");

    println!("{}", s);

    let s2 = s.clone();

    println!("s = {}, s2 = {}", s, s2);

    takes_ownership(s);

    let x = 5;

    makes_copy(x);

    let s3 = gives_ownership();

    let s4 = String::from("hello");

    let s5 = takes_and_gives_back(s4);

    let len = calculate_length(&s5);

    println!("The length of '{}' is {}.", s5, len);

    let s6 = String::from("hello");

    let r1 = &s6;
    let r2 = &s6;
    let r3 = &mut s6;
    println!("{}, {}", r1, r2);
}

fn takes_ownership (some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let some_string = String::from("Hello");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn calculate_length(s: &String) -> usize {
    s.len()
}