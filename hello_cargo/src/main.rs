fn main() {
    println!("Hello, world!");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);
}