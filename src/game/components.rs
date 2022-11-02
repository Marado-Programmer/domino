use std::fmt::{Debug, Display, Formatter, Result};
use std::io;

#[derive(Clone, Copy)]
pub struct Stone(pub u8, pub u8);

impl Debug for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}|{}]", self.0, self.1)
    }
}

impl Display for Stone {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "[{}|{}]", self.0, self.1)
    }
}

#[derive(Debug)]
pub enum AILvl {
    Easy,
}

#[derive(Debug)]
pub enum PlayerType {
    Human,
    AI(AILvl),
}

#[derive(Debug)]
pub struct Player {
    typ: PlayerType,
    pub stones: Vec<Option<Stone>>,
}

impl Player {
    pub fn new(typ: PlayerType, stones: Vec<Option<Stone>>) -> Self {
        Self { typ, stones }
    }

    pub fn play(&mut self) -> Option<Stone> {
        if let PlayerType::AI(lvl) = &self.typ {
            self.ai_play(lvl)
        } else {
            self.human_play()
        }
    }

    fn human_play(&mut self) -> Option<Stone> {
        let mut prompt = String::from("Your Stones:");

        for i in self.stones.iter().filter(|x| x.is_some()) {
            let stone = i.unwrap_or_else(|| Stone(u8::MAX, u8::MAX));
            prompt.push_str(&format!(" {stone}")[..]);
        }

        println!("{}", prompt);

        let mut index: usize;

        loop {
            let mut input = String::new();

            if let Err(_) = io::stdin().read_line(&mut input) {
                panic!("Could not read_line")
            }

            index = match input.trim().parse() {
                Ok(i) => i,
                Err(_) => {
                    continue;
                }
            };

            if index < self.stones.len() && self.stones.get(index).is_some() {
                break;
            }
        }

        self.stones[index].take()
    }

    fn ai_play(&self, _lvl: &AILvl) -> Option<Stone> {
        //println!("AI Stones: {:?}", self.stones);

        self.stones[3]
    }
}
