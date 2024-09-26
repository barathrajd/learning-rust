#[derive(Debug)]
struct User {
    active:bool,
    username: String,
    email:String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: false,
        username: String::from("Barath"),
        email: String::from("barathdeva757@gmail.com"),
        sign_in_count: 100,
    };
    println!("User 1");
    println!("Active: {:?}", user1.active);
    println!("Username: {:?}", user1.username);
    println!("Email: {:?}", user1.email);
    println!("SignInCount: {:?}", user1.sign_in_count);

    let user2 = User {
        username: String::from("Raj"),
        email: String::from("raj@gmail.com"),
        ..user1
    };
    println!("User 2");
    println!("Active: {:?}", user2.active);
    println!("Username: {:?}", user2.username);
    println!("Email: {:?}", user2.email);
    println!("SignInCount: {:?}", user2.sign_in_count);

}
