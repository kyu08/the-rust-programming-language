fn main() {
    let u = build_user(String::from("john"), 32);
    println!("name: {}, age: {}, active: {}", u.name, u.age, u.active);

    let u = add_age(u);
    println!("name: {}, age: {}, active: {}", u.name, u.age, u.active);
}

struct User {
    name: String,
    age: u32,
    active: bool,
}

fn build_user(name: String, age: u32) -> User {
    User {
        name,
        age,
        active: true,
    }
}

fn add_age(u: User) -> User {
    User {
        age: u.age + 1,
        ..u
    }
}
