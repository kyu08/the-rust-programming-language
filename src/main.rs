fn main() {
    let mut u = User::build_user(String::from("john"), 32);
    println!("name: {}, age: {}, active: {}", u.name, u.age, u.active);

    println!("is_active_adult: {}", u.is_active_adult());

    u.add_age();
    println!("name: {}, age: {}, active: {}", u.name, u.age, u.active);
}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
    active: bool,
}

impl User {
    fn build_user(name: String, age: u32) -> User {
        User {
            name,
            age,
            active: true,
        }
    }
    fn is_active_adult(&self) -> bool {
        self.active && 20 <= self.age
    }

    fn add_age(&mut self) {
        self.age += 1;
    }
}
