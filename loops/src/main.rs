macro_rules! object {
    {} => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:ident: $( $cont:tt )*) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:literal: $( $cont:tt )*) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) [$key:expr]: $( $cont:tt )*) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:expr => $value:tt, $( $cont:tt )+) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:expr => $value:tt,) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:expr => $value:tt) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:expr => $value:expr, $( $cont:tt )+) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:expr => $value:expr,) => { ... };
    (@ENTRY($( $k:expr => $v:expr, )*) $key:expr => $value:expr) => { ... };
    (@END $( $k:expr => $v:expr, )*) => { ... };
    ($key:tt: $( $cont:tt )+) => { ... };
    ($( $k:expr => $v:expr, )*) => { ... };
    ($( $k:expr => $v:expr ),*) => { ... };
}

fn main() {
    loop {
        println!("again!");
        break;
    }

    let mut number = 3;
    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}
