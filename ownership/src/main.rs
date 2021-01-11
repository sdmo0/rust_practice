fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    print_string(s);

    // println!("{}", s);

    first();

    second();

    third();

    forth();

    fifth();

    sixth();
}

fn print_string(str: String) {
    println!("{}", str);
}

fn first() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}

fn second() {
    let s1 = String::from("hello, world!");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn third() {
    let s1 = String::from("hellow~");
    let len = calculate_length2(&s1);

    println!("the length of '{}' is {}.", s1, len);
}

fn calculate_length2(s: &String) -> usize {
    println!("dereferencing: {}", *s);
    s.len()
}

fn forth() {
    let mut s = String::from("hello~");
    let mut s2 = String::from("test");
    s = String::from("test");

    let mut a = &mut s;
    a = &mut s2;

    println!("{}", a);

    change(&mut s);
    println!("mutble string: {}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

fn fifth() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
}

fn gives_ownership() -> String {
    let some_string = String::from("who's");
    some_string
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}

fn sixth() {
    let str1 = String::from("this is a string");
    let ref_str1 = &str1;

    println!("string is: {}", str1);
    println!("first word index: {}", first_word(ref_str1));
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
