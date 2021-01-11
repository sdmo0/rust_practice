fn main() {
    let x = 0x5000_abcd_1234_0987u64;
    println!("The value of x is: {}", x);
    let x = 0o76 + 8;
    println!("The value of x is: {}", x);
    let x: isize = 6isize;
    println!("The value of x is: {}", x);
    let x = 0b11011_001;
    println!("The value of x is: {}", x);
    let x = b'8' + 9u8;
    println!("The value of x is: {}", x);
    let x = 8.7f32;
    println!("The value of x is: {}", x);
    // let x = 1.7f64 + 8.7f32;
    // println!("The value of x is: {}", x);

    let x: char = '한';
    println!("The value of x is: {}", x);

    // let x = '한' + "글";
    // let x = "한" + '글';
    println!("The value of x is: {}", x);

    let tup: (i32, f64, i8) = (500, 7.5, 127);
    println!("The value of tup is: {:?}", tup);

    let (x, y, z) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);

    println!("The value of tup.0 is: {}", tup.0);
    println!("The value of tup.1 is: {}", tup.1);
    println!("The value of tup.2 is: {}", tup.2);

    let months = [
        "January", "Feb", "Mar", "April", "May", "June", "July", "Aug", "Sep", "Oct", "Nov", "Dec",
    ];
    let first = months[0];
    let index = 1;
    let second = months[index];

    println!("first = {}, second = {}", first, second);

    another_function();
}

fn another_function() {
    println!("okokok another function");
}
