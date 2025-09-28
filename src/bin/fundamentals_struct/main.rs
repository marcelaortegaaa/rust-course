//Structs Demo
// Spent 2 hours figuring out what an attribute is :)
//
// [X] Add associated function to named 'from_email' to 'User'. Takes email as a parameter and returns new `User` instance with 'username' derived from email address.
// Done but I think it could be improved. Need to remove Some() from User.

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
    ) -> Person {
        Person {
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
    username: Option<String>,
    email: String,
    uri: String,
    active: bool,
}

impl User {
    fn new(username: Option<String>, email: String, uri: String) -> Self {
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
            self.username = Some(username.to_string());
        }
    }
}

fn main() {
    let new_person = Person::new(
        String::from("John"),
        String::from("Wick"),
        61,
        String::from("johnwick@movie.com"),
        String::from("+1 234 5678"),
    );

    let mut new_user = User::new(
        None,
        String::from("jwick@mail.com"),
        String::from("jwick.com"),
    );

    println!("{:#?}", new_person);
    println!("This person's full name is {}", new_person.full_name());

    new_user.set_username();
    println!("Hello, {:#?}!", new_user.username);
    println!(
        "Account {:#?} status is: {}",
        new_user.username, new_user.active
    );

    new_user.deactivate();
    println!(
        "Account {:#?} status is: {}",
        new_user.username, new_user.active
    );
    println!("{:#?}", new_user.username);
}
