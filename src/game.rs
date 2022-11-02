mod components;
use self::components::{Player, PlayerType, Stone};
use rand::Rng;

const STONE_MAX_DOTS: u8 = 6;

const AMOUNT_STONES: u8 = (STONE_MAX_DOTS + 1) * (STONE_MAX_DOTS + 2) / 2;

#[derive(Debug)]
pub struct Game {
    domino_set: Vec<Option<Stone>>,
    players: Vec<Player>,
    started: bool,
    playing: usize,
    priority: Option<usize>,
}

impl Game {
    pub fn start(num_players: u8, num_humans: Option<u8>) {
        if num_players < 2 || num_players > 4 {
            panic!("Players: 2--4, you selected {}", num_players);
        }

        let num_humans = num_humans.unwrap_or(0);

        if num_players < num_humans {
            panic!("Number of human players: 2--{}, you selected {}", num_players, num_humans);
        }

        let players: Vec<Player> = Vec::with_capacity(num_players as usize);
        let domino_set = Self::new_domino_set(AMOUNT_STONES as usize);

        let mut game = Self {
            domino_set,
            players,
            started: false,
            playing: 0,
            priority: None,
        };
        game.add_players(num_players, num_humans, AMOUNT_STONES);

        game.give_initial_stones();

        game.create_priority();

        loop {
            game.run();

            if !game.started {
                break;
            }
        }
    }

    fn new_domino_set(size: usize) -> Vec<Option<Stone>> {
        let mut domino_set: Vec<Option<Stone>> = Vec::with_capacity(size);

        let mut i = STONE_MAX_DOTS;
        for j in (0..=STONE_MAX_DOTS).rev() {
            for k in (0..=i).rev() {
                domino_set.push(Some(Stone(j, k)));
            }

            if i > 0 {
                i -= 1;
            }
        }

        domino_set
    }

    fn add_players(&mut self, num_players: u8, num_humans: u8, num_tiles: u8) {
        for i in 0..num_players {
            let p = Player::new(
                if i < num_humans {
                    PlayerType::Human
                } else {
                    PlayerType::AI(0)
                },
                Vec::with_capacity((num_tiles / num_players) as usize),
            );

            self.players.push(p);
        }
    }

    fn give_initial_stones(&mut self) {
        for i in 0..self.players.len() {
            for _ in 1..=7 {
                self.give_stone(i);
            }
        }
    }

    fn give_stone(&mut self, p: usize) {
        loop {
            let i = rand::thread_rng().gen_range(0..self.domino_set.len());

            if let Some(_) = self.domino_set[i] {
                self.players[p].stones.push(self.domino_set[i].take());
                break;
            }
        }
    }

    fn player_playing(&mut self) -> &mut Player {
        let i = self.playing;

        self.playing = (i + 1) % self.players.len();

        &mut self.players[i]
    }

    fn create_priority(&mut self) {
        'dots: for i in (0..=STONE_MAX_DOTS).rev() {
            for (j, p) in self.players.iter().enumerate() {
                for (k, s) in p.stones.iter().enumerate() {
                    if let Some(s) = s {
                        if s.0 == s.1 && s.0 == i {
                            self.playing = j;
                            self.priority = Some(k);
                            break 'dots;
                        }
                    }
                }
            }
        }
    }

    fn run(&mut self) {
        let priority = self.priority.take();

        let p = self.player_playing();

        if let Some(priority) = priority {
            p.stones[priority].take();
            return;
        } else {
            p.play().take();
        }

        self.started = false;
    }
}
