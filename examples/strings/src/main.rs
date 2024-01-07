fn print_str(s: &str) {
    println!("{}", s);

    let mut new_string = s.to_string();
    new_string.push_str(" some other string");
    println!("{}", new_string);

    let new_string_2 = format!("{} other stuff here", s);
    println!("{}", new_string_2);
}

fn print_string(mut s: String) {
    println!("{}", s);
}

fn main() {
    let s = "hellow, world!"; // String slice
    print_str(s);

    // String is growable and mutable whereas str is not.
    // String is owned by the code that creates it
    let mut salutation = String::from("hello");
    print_string(salutation);
}
