use core::num;

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

    println!("入れ替えたいカードの番号を入力してください（例: 1 2 3）");
    // ユーザーからの入力を入れるための変数
    let mut input = String::new();
    // ユーザーから入力を変数に書き込み
    std::io::stdin().read_line(&mut input).unwrap();

    // 選ばれたカードを山札から引いたカードで置き換える
    // 扱いやすいようにVecに変換する
    let numbers: Vec<usize> = input
        .split_whitespace() // 文字列を空白区切りで分割する
        .map(|x| x.parse().unwrap()) // 文字列を数値に変換する
        .collect::<Vec<usize>>();

    // 与えられた数字の箇所をデッキから取り出したカードに置き換える
    for number in numbers {
        hand[number - 1] = deck.pop().unwrap();
    }

    // 手札をソート
    hand.sort_by(|a, b| a.rank.cmp(&b.rank));
    // 手札を表示
    println!("--Hand--");
    for (i, card) in hand.iter().enumerate() {
        println!("{:}: {:?} {:}", i + 1, card.suit, card.rank);
    }
}
