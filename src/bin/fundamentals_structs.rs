//Structs Demo
// Spent 2 hours figuring out what an attribute is :)

#[derive(Debug)]
struct Person {
    first_name: String,
    last_name: String,
    age: u8,
    email: String,
    phone_number: String,
}

impl Person {
    fn new(
        first_name: String,
        last_name: String,
        age: u8,
        email: String,
        phone_number: String,
    ) -> Self {
        Self {
            first_name,
            last_name,
            age,
            email,
            phone_number,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}

struct User {
    username: String,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(email: String, uri: String) -> Self {
        let username = match email.split_once('@') {
            Some((user, _rest)) => user.to_string(),
            None => email.clone(),
        };

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

    fn set_username(&mut self) {
        if let Some((username, _)) = self.email.split_once("@") {
            self.username = username.to_string();
        }
    }
}

fn main() {
    let new_person = Person::new(
        "John".into(),
        "Wick".into(),
        61,
        "johnwick@movie.com".into(),
        "+1 234 5678".into(),
    );

    let mut new_user = User::new("jwick@mail.com".into(), "jwick.com".into());

    println!("{:#?}", new_person);
    println!("This person's full name is {}", new_person.full_name());
    println!(
        "{}, {}, {}",
        new_person.age, new_person.email, new_person.phone_number
    );

    new_user.set_username();
    println!("Hello, {}!", new_user.username);
    println!(
        "Account {} ({}) status is: {}",
        new_user.username, new_user.uri, new_user.active
    );

    new_user.deactivate();
    println!(
        "Account {} status is: {}",
        new_user.username, new_user.active
    );

    println!("This is the struct: {:#?}", new_person);
}
