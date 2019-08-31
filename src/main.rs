fn main() {
    let s = String::from("book");

    let ss = pluralize(&s);

    println!(
        "I have one {}, you have two {}",
        s,
        ss
        );
}

fn pluralize(x: &str) -> String {
    return format!("{}s",x);
}
