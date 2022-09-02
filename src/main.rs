use rand::prelude::*;
use std::{fmt, collections::HashMap};

struct Game {
    players: Vec<Player>,
    /// Use the Card to find out how much are remaining
    deck: HashMap<Card, usize>,
    discard_pile: Vec<Card>,
}

struct Player {
    name: String,
    coins: u8,
    cards: Vec<Card>,
    current_play: Option<Play>,
}

struct Play {
    card: Card,
    is_lie: bool,
    target: Box<Player>,
}

#[derive(PartialEq, Eq, Hash)]
struct Card {
    character: Character,
    action: Action,
    effect: Effect,
    counteraction: Counteraction,
}

#[derive(PartialEq, Eq, Hash)]
enum Character {
    X,
    Duke,
    Assassin,
    Ambassador,
    Captain,
    Contessa,
}

#[derive(PartialEq, Eq, Hash)]
enum Action {
    X,
    Income,
    ForeignAid,
    Coup,
    Tax,
    Assassinate,
    Exchange,
    Steal,
}

#[derive(PartialEq, Eq, Hash)]
enum Effect {
    X,
    /// Take coins from bank u8 being the amount
    Take(u8),
    /// Kill player with the u8 holding the amount of coins it costs
    Kill(u8),
    /// Exchange cards with court deck
    ExchangeCards,
    TakeTwoFromOtherPlayer,
}

#[derive(PartialEq, Eq, Hash)]
enum Counteraction {
    X,
    BlocksForeignAid,
    BlocksStealing,
    BlocksAssasination,
}

impl fmt::Display for Character {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match *self {
            Character::X => "-----",
            Character::Duke => "Duke",
            Character::Assassin => "Assassin",
            Character::Ambassador => "Ambassador",
            Character::Captain => "Captain",
            Character::Contessa => "Contessa",
        };

        write!(f, "{name}")
    }
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let action = match *self {
            Action::X => "X",
            Action::Income => "Income",
            Action::ForeignAid => "Foreign Aid",
            Action::Coup => "Coup",
            Action::Tax => "Tax",
            Action::Assassinate => "Assassinate",
            Action::Exchange => "Exchange",
            Action::Steal => "Steal",
        };

        write!(f, "{action}")
    }
}

impl fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let effect: String = match *self {
            Effect::X => String::from("X"),
            Effect::Take(coins) => format!("Take {coins} coins"),
            Effect::Kill(coins) => format!("Pay {coins} coins.\nChoose a player to loose influence"),
            Effect::ExchangeCards => String::from("Exchange cards with Court Deck"),
            Effect::TakeTwoFromOtherPlayer => String::from("Take 2 coins from another player")
        };

        write!(f, "{effect}")
    }
}

impl fmt::Display for Counteraction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let counteraction = match *self {
            Counteraction::X => "X",
            Counteraction::BlocksForeignAid => "Blocks Foreign Aid",
            Counteraction::BlocksStealing => "Blocks Stealing",
            Counteraction::BlocksAssasination => "Blocks Assassination",
        };

        write!(f, "{counteraction}")
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "Character: {}\nAction: {}\nEffect: {}\nCounteraction: {}",
            self.character,
            self.action,
            self.effect,
            self.counteraction
        )
    }
}

impl Player {
    fn print_cards(&self) {
        println!("----- Your Hand -----\n");
        for card in &self.cards {
            println!("{}\n", card);
        }
        println!("----- Your Hand -----");
    }
}

impl Card {
    fn duke() -> Card {
        Card { 
            character: Character::Duke, 
            action: Action::Tax, 
            effect: Effect::Take(3), 
            counteraction: Counteraction::BlocksForeignAid, 
        }
    }
    fn assassin() -> Card {
        Card { 
            character: Character::Assassin, 
            action: Action::Assassinate, 
            effect: Effect::Kill(3), 
            counteraction: Counteraction::X, 
        }
    }
    fn ambassador() -> Card {
        Card { 
            character: Character::Ambassador, 
            action: Action::Exchange, 
            effect: Effect::ExchangeCards, 
            counteraction: Counteraction::BlocksStealing, 
        }
    }
    fn captain() -> Card {
        Card { 
            character: Character::Captain, 
            action: Action::Steal, 
            effect: Effect::TakeTwoFromOtherPlayer, 
            counteraction: Counteraction::BlocksStealing, 
        }
    }
    fn contessa() -> Card {
        Card { 
            character: Character::Contessa, 
            action: Action::X, 
            effect: Effect::X, 
            counteraction: Counteraction::BlocksAssasination, 
        }
    }

    fn income() -> Card {
        Card { 
            character: Character::X, 
            action: Action::Income, 
            effect: Effect::Take(1), 
            counteraction: Counteraction::X, 
        }
    }
    fn foreign_aid() -> Card {
        Card { 
            character: Character::X, 
            action: Action::ForeignAid, 
            effect: Effect::Take(2), 
            counteraction: Counteraction::X, 
        }
    }
    fn coup() -> Card {
        Card { 
            character: Character::X, 
            action: Action::Coup, 
            effect: Effect::Kill(7), 
            counteraction: Counteraction::X, 
        }
    }
}

impl Game {
    fn new() -> Game {
        let mut deck = HashMap::new();

        // Insert cards into deck
        deck.insert(Card::duke(), 3);
        deck.insert(Card::assassin(), 3);
        deck.insert(Card::ambassador(), 3);
        deck.insert(Card::captain(), 3);
        deck.insert(Card::contessa(), 3);

        Game { 
            players: Vec::new(),
            deck,
            discard_pile: Vec::new(),
        }
    }

    fn add_player(&mut self, name: String, cards: Vec<Card>) {
        self.players.push(Player {
            name,
            coins: 0,
            cards,
            current_play: None,
        });
    }

    /// Draw a full hand (2 cards from deck, random)
    fn full_draw(&mut self) -> Vec<Card> {
        vec![
            Card::income(),
            Card::foreign_aid(),
            Card::coup(),

            self.random(), 
            self.random(),
        ]
    }

    /// Draw a random card from deck
    fn random(&mut self) -> Card {
        if self.deck.is_empty() {
            println!("Deck is empty flipping discard pile");

            return Card::duke();
        }

        loop {
            let n = thread_rng().gen_range(0..=5);

            let card = match n {
                0 => Card::duke(),
                1 => Card::assassin(),
                3 => Card::ambassador(),
                4 => Card::captain(),
                5 => Card::contessa(),

                _ => Card::contessa(),
            };

            // If there is more of that card
            if let Some(_) = self.deck.get(&card) {
                let num = self.deck.get_mut(&card).unwrap();

                *num -= 1;

                if *num == 0 {
                    self.deck.remove(&card).unwrap();
                }

                return card;
            } else { // There is no more of the card
                continue;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    let names: Vec<&str> = vec!["Nate", "You", "Me", "Mitch", "Fitch"];
let mut i = 0;
    for name in names {
        let cards = game.full_draw();

        game.add_player(String::from("Nate"), cards);
        
        game.players[i].print_cards();

        i += 1;
    }
    
    game.players[0].play(0, None)
}