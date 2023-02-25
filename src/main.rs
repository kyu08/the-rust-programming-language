fn main() {
    let u = User {
        name: String::from("john"),
        age: 32,
    };

    let mut u_mutable = u;
    u_mutable.age += 1;
    println!("name: {}, age: {}", u_mutable.name, u_mutable.age);
}

struct User {
    name: String,
    age: u32,
}
