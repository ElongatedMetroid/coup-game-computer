use std::fmt;

struct Player {
    name: String,
    coins: u8,
    cards: Vec<Card>,
}

struct Card {
    character: Character,
    action: Action,
    effect: Effect,
    counteraction: Counteraction,
    target: Option<Player>,
    is_lie: bool,
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
            Effect::Kill(coins) => format!("Pay {coins} coins.\n Choose a player to loose influence"),
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
    fn duke() -> Card {
        Card { 
            character: Character::Duke, 
            action: Action::Tax, 
            effect: Effect::Take(3), 
            counteraction: Counteraction::BlocksForeignAid, 
            target: None, 
            is_lie: false 
        }
    }
}

fn main() {
    let duke = Card::duke();

    println!("{}", duke);
}