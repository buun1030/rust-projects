fn main() {
    println!("Hello, world!");

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let mut s1 = String::from("hello world");
    let _s2 = s1.clone();
    let word = first_word(&s1);
    println!("s1 = {}", word);

    let mut num = 5;
    let r1 = &mut num;
    let r2 = &mut num;
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}