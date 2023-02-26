fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    println!("{}", third);

    match v.get(20) {
        Some(v) => println!("{}", v),
        None => println!("none"),
    }
}
