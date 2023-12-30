struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: String, email: String, uri: String) -> Self {
        Self {
            username,
            email,
            uri,
            active: true,
        }
    }
}

fn main() {
    let new_user = User::new(
        String::from("johndoe"),
        String::from("abc@cde.com"),
        String::from("https://johndoe.com"),
    );
    println!("Hello, {}!", new_user.username);
}
