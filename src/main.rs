use rand::prelude::*;
use std::fmt;

struct Game {
    players: Vec<Player>,
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

struct Card {
    character: Character,
    action: Action,
    effect: Effect,
    counteraction: Counteraction,
}

enum Character {
    X,
    Duke,
    Assassin,
    Ambassador,
    Captain,
    Contessa,
}

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

impl Card {
    fn random() -> Card {
        let n = thread_rng().gen_range(0..=5);

        match n {
            0 => Card::duke(),
            1 => Card::assassin(),
            3 => Card::ambassador(),
            4 => Card::captain(),
            5 => Card::contessa(),

            _ => Card::contessa(),
        }
    }

    fn full_draw() -> Vec<Card> {
        vec![Card::random(), Card::random()]
    }

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
}

impl Game {
    fn new() -> Game {
        Game { 
            players: Vec::new()
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
}

fn main() {
    let mut game = Game::new();

    game.add_player(String::from("Nate"), Card::full_draw());

    println!("{}\n\n{}", game.players[0].cards[0], game.players[0].cards[1]);
}