fn main() {
    another_function(5);

    let x = plus_one(5) + plus_two(6);
    println!("x = {}", x);
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn plus_one(x: i32) -> i32 {
    return x + 1;
}

fn plus_two(x: i32) -> i32 {
    x + 2
}
