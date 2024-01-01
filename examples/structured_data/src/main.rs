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

#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let person = Person {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        age: 25,
        email: Some("abc@cde.com".to_string()),
        phone: None,
    };

    let first_name = "Jane".to_string();
    let last_name = "Doe".to_string();
    let age = 26;
    let email = Some("jane@cde.com".to_string());
    let phone = None;

    let person_2 = Person {
        first_name,
        last_name,
        age,
        email,
        phone,
    };

    let point = Point(1, 2, 3);

    println!("{:?}", person);
    println!("Full name: {}", get_full_name(&person));
    println!("The person's first name is: {}", person.first_name);

    println!("{:?}", person_2);

    println!("{:?}", point);
}
