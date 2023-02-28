struct NewsFeed {}
impl Runner for NewsFeed {}
impl Summary for NewsFeed {
    fn summarize(&self) -> String {
        String::from("news!")
    }
}

struct Tweet {}
impl Runner for Tweet {}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from("tweet!")
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

trait Runner {}

fn main() {
    let n = NewsFeed {};
    let t = Tweet {};

    say(n);
    say(t);
}

fn say<T>(summary: T)
where
    T: Summary + Runner,
{
    println!("{}", summary.summarize());
}
