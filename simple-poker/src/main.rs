use rand::seq::SliceRandom;

// enum型
#[derive(Debug, Clone, Copy, PartialEq)]
enum Suit {
    Club,
    Diamond,
    Heart,
    Spade,
}

// 構造体
#[derive(Debug, Clone, Copy, PartialEq)]
struct Card {
    suit: Suit,
    rank: i32,
}

fn main() {
    // Vev の用意
    let mut deck: Vec<Card> = Vec::new();
    let suits = [Suit::Club, Suit::Diamond, Suit::Heart, Suit::Spade];

    // Deck を作成
    for suit in suits {
        for rank in 1..=13 {
            // Vecにカードを入れる
            deck.push(Card { suit, rank });
        }
    }
    // deckをシャッフル
    let mut rng = rand::thread_rng();
    deck.shuffle(&mut rng);
    println!("{:?}", deck);

    // 手札用のVecの用意
    let mut hand: Vec<Card> = Vec::new();
    // ５枚のカードを引く
    for _ in 0..5 {
        hand.push(deck.pop().unwrap());
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    // 手札を表示
    println!("--Hand--");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }
}
