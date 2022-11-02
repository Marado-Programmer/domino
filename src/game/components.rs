use std::fmt::Debug;

#[derive(Clone, Copy)]
pub struct Stone(pub u8, pub u8);

impl Debug for Stone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}|{}]", self.0, self.1)
    }
}

#[derive(Debug)]
pub enum PlayerType {
    Human,
    AI(u8),
}

#[derive(Debug)]
pub struct Player {
    typ: PlayerType,
    pub stones: Vec<Option<Stone>>,
}

impl Player {
    pub fn new(typ: PlayerType, stones: Vec<Option<Stone>>) -> Self {
        Player {
            typ,
            stones,
        }
    }

    pub fn play(&self) -> Option<Stone> {
        if let PlayerType::Human = self.typ {
            self.human_play()
        } else {
            self.ai_play()
        }
    }

    fn human_play(&self) -> Option<Stone> {
        println!("Your Stones: {:?}", self.stones);

        self.stones[1]
    }

    fn ai_play(&self) -> Option<Stone> {
        self.stones[3]
    }
}
