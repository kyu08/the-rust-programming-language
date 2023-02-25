fn main() {
    let mut u = User {
        name: String::from("john"),
        age: 32,
    };

    u.age += 1;
    println!("name: {}, age: {}", u.name, u.age);
}

struct User {
    name: String,
    age: u32,
}
