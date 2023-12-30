#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: Option<String>,
    phone: Option<String>,
}

fn get_full_name(person: &Person) -> String {
    format!("{} {}", person.first_name, person.last_name)
}

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: Some("abc@cde.com".to_string()),
        phone: None,
    };
    println!("{:?}", person);
    println!("Full name: {}", get_full_name(&person));
}
