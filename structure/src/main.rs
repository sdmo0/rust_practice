struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername"),
        active: true,
        sign_in_count: 1,
    };

    user1.active = false;

    // println!("user: {:?}", build_user("this email", "this user", false, 64));
}

fn build_user(email: String, username: String, active: bool, sign_in_count: u64) -> User {
    User {
        email,
        username,
        active,
        sign_in_count,
    }
}
