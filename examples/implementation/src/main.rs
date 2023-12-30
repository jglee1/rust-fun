#[derive(Debug)]
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
    fn deactivate(&mut self) {
        self.active = false;
    }
    fn from_email(email: String) -> Self {
        let username = email.split('@').next().unwrap();
        Self {
            username: String::from(username),
            email,
            uri: String::from(""),
            active: true,
        }
    }
}

fn main() {
    let mut new_user = User::new(
        String::from("johndoe"),
        String::from("abc@cde.com"),
        String::from("https://johndoe.com"),
    );
    println!("Hello, {}!", new_user.username);
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );
    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    let email_2 = String::from("steve@cde.com");
    let new_user_2 = User::from_email(email_2);
    println!("Hello, {}!", new_user_2.username);
    println!("{:?}", new_user_2);
}
