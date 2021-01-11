fn main() {
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    println!("Hello, world!");

    let yo = first_word(&s);

    // s.clear();

    println!("first word: {}", first_word(&s));
    println!("first word for hello: {}", first_word(hello));

    s.clear();
}

fn first_word(s: &str) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}

fn second() {
    let my_string = String::from("Hello World");
    let word = first_word(&my_string[..]);
    let word2 = first_word(&my_string);

    let my_string_literal = "Hello World";
    let word = first_word(&my_string_literal);
    let word2 = first_word(my_string_literal);
}
