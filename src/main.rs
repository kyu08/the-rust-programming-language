fn main() {
    let vec1 = vec![1, 2, 3, 4, 5];
    println!("largest num: {}", largest(&vec1));

    let vec2 = vec!["a", "b"];
    println!("largest num: {}", largest(&vec2));
}

fn largest<T: PartialOrd>(vec: &Vec<T>) -> &T {
    let mut largest = match vec.first() {
        Some(v) => v,
        None => panic!("vec should not empty!"),
    };

    for item in vec {
        if *largest < *item {
            largest = item;
        }
    }

    largest
}
