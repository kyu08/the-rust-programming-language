struct NewsFeed {}
impl Summary for NewsFeed {
    fn summarize(&self) -> String {
        String::from("news!")
    }
}

struct Tweet {}
impl Summary for Tweet {
    fn summarize(&self) -> String {
        String::from("tweet!")
    }
}

trait Summary {
    fn summarize(&self) -> String;
}

fn main() {
    let n = NewsFeed {};
    let t = Tweet {};

    say(n);
    say(t);
}

fn say(summary: impl Summary) {
    println!("{}", summary.summarize());
}
