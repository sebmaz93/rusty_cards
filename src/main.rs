#[derive(Debug)]
struct Deck {
    cards: Vec<String>,
}

fn main() {
    let deck: Deck = Deck { cards: vec![
        "two of hearts",
        ""
    ] };

    println!("Here is your deck: {:?}", deck);
}
