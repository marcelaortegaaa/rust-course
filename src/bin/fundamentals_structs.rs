//Structs Demo
// Spent 2 hours figuring out what an attribute is :)

#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    username: String,
    email: String,
    age: u8,
    active: bool,
}

impl User {
    fn new(first_name: String, last_name: String, email: String, age: u8) -> Self {
        let username = match email.split_once('@') {
            Some((user, _)) => user.to_string(),
            None => email.clone(),
        };

        Self {
            first_name,
            last_name,
            username,
            email,
            age,
            active: true,
        }
    }

    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn deactivate(&mut self) {
        self.active = false;
        println!(
            "The status for the account {} is now {}",
            self.username, self.active
        );
    }

    fn greet(&self) {
        println!("Hello, {}.", self.full_name());
    }
}

fn main() {
    let mut new_user = User::new(
        "John".to_string(),
        "Wick".to_string(),
        "johnwick@movie.com".to_string(),
        61,
    );

    new_user.greet();
    new_user.deactivate();
    println!("age: {}, email: {}", new_user.age, new_user.email)
}
