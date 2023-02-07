mod input;

use std::{fmt::Display, ops::Not};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Player {
    None,
    Cross,
    Circle,
}

impl Not for Player {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Player::Circle => Player::Cross,
            Player::Cross => Player::Circle,
            Player::None => Player::None,
        }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Player::Circle => write!(f, "O"),
            Player::Cross => write!(f, "X"),
            Player::None => write!(f, "None"),
        }
    }
}

#[derive(Debug)]
pub enum TTTError {
    InvalidMove,
}
#[derive(Clone, Copy)]
pub struct Game {
    board: [[Player; 3]; 3],
    turn: Player,
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.render())
    }
}

impl Game {
    pub fn new() -> Game {
        Game {
            board: [[Player::None; 3]; 3],
            turn: Player::Cross,
        }
    }

    fn render(&self) -> String {
        let mut out = String::new();

        out.push_str("  0 1 2\n");
        out.push_str("  ------\n");
        for y in self.board {
            out.push_str("0|");
            for x in y {
                match x {
                    Player::None => out.push_str("  "),
                    Player::Circle => out.push_str("O "),
                    Player::Cross => out.push_str("X "),
                }
            }
            out.pop();
            out.push_str("|\n")
        }
        out.push_str("  ------");
        out
    }

    pub fn make_move(&self, x: u8, y: u8) -> Result<Game, TTTError> {
        let mut game = self.clone();

        if x > 2 || y > 2 {
            return Err(TTTError::InvalidMove);
        }

        let cell = self.board[y as usize][x as usize];

        match cell {
            Player::None => game.board[y as usize][x as usize] = self.turn,
            _ => return Err(TTTError::InvalidMove),
        };

        game.turn = !self.turn;

        println!("{}", game.check_win());

        Ok(game)
    }

    fn check_win(&self) -> Player {
        if self.board[0][0] == self.board[0][1] && self.board[0][1] == self.board[0][2] {
            self.board[0][0]
        } else if self.board[1][0] == self.board[1][1] && self.board[1][1] == self.board[1][2] {
            self.board[1][0]
        } else if self.board[2][0] == self.board[2][1] && self.board[2][1] == self.board[2][2] {
            self.board[2][0]
        } else if self.board[0][0] == self.board[1][0] && self.board[1][0] == self.board[2][0] {
            self.board[0][0]
        } else if self.board[0][1] == self.board[1][1] && self.board[1][1] == self.board[2][1] {
            self.board[0][1]
        } else if self.board[0][2] == self.board[1][2] && self.board[1][2] == self.board[2][2] {
            self.board[0][2]
        } else if self.board[0][0] == self.board[1][1] && self.board[1][1] == self.board[2][2] {
            self.board[0][0]
        } else if self.board[0][2] == self.board[1][1] && self.board[1][1] == self.board[2][0] {
            self.board[0][2]
        } else {
            Player::None
        }
    }

    pub fn player_make_move(&self) -> Result<Game, TTTError> {
        println!("What's {}'s move?", self.turn);
        let input = input::get_input();

        let nums: Vec<u8> = input
            .trim()
            .split(" ")
            .map(|num| {
                println!("{num}");
                num.parse::<u8>().unwrap()
            })
            .collect();

        self.make_move(nums[0], nums[1])
    }
}
