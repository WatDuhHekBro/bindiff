use colour::*;
use regex::Regex;

fn main() {
    let re = Regex::new(r"\d{4}-\d{2}-\d{2}").unwrap();
    let text = "some date which is 2020-12-19";

    match re.find(text) {
        Some(b) => {
            let c = &text[b.start()..b.end()];
            red!(c);
        }
        None => println!("None"),
    }
}
