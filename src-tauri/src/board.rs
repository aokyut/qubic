// use std::collections::VecDeque;
use proconio::input;
use rand::Rng;
use std::fmt;
use std::{
    borrow::BorrowMut,
    cell::{Ref, RefCell},
    collections::HashMap,
};

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum Player {
    Black,
    White,
}

impl Player {
    pub fn next(&self) -> Self {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }

    pub fn from_u64(board: u64) -> Self {
        if board % 2 == 0 {
            return Player::Black;
        } else {
            return Player::White;
        }
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Board {
    pub black: u64,
    pub white: u64,
    player: Player,
}

impl Board {
    pub fn new() -> Self {
        return Board {
            black: 0,
            white: 0,
            player: Player::Black,
        };
    }
    pub fn next(&self, action_id: u8) -> Self {
        let board = self.black | self.white;
        let action_bitboard: u64 =
            (0x0001000100010001u64 << action_id) & ((!board << 16) ^ (!board));
        match self.player {
            Player::Black => Board {
                black: self.black | action_bitboard,
                white: self.white,
                player: self.player.next(),
            },
            Player::White => Board {
                black: self.black,
                white: self.white | action_bitboard,
                player: self.player.next(),
            },
        }
    }

    pub fn is_draw(&self) -> bool {
        return (self.black | self.white) == 0xffffffffffffffff;
    }

    pub fn is_win(&self) -> bool {
        match self.player {
            Player::White => _is_win_board(self.black),
            Player::Black => _is_win_board(self.white),
        }
    }

    pub fn clone(&self) -> Self {
        return Board {
            black: self.black,
            white: self.white,
            player: self.player.next(),
        };
    }

    pub fn action_mask(&self) -> u64 {
        let board = self.black | self.white;
        return board >> 48;
    }

    pub fn valid_actions(&self) -> Vec<u8> {
        let mut actions = Vec::<u8>::new();
        let board = (self.black | self.white) >> 48;
        for i in 0..16u8 {
            if (board >> i) & 1 == 0 {
                actions.push(i);
            }
        }

        return actions;
    }

    pub fn minimax_action(&self, depth: u8) -> u8 {
        let mut rng = rand::thread_rng();
        if depth == 1 {
            let actions = self.valid_actions();
            for action in actions.iter() {
                let next_board = self.next(*action);
                if next_board.is_win() {
                    return *action;
                } else if next_board.is_draw() {
                    return *action;
                }
            }
            return actions[rng.gen::<usize>() % actions.len()];
        } else {
            let mut actions: Vec<u8> = Vec::new();
            let mut max_val: i8 = -2;
            for action in self.valid_actions() {
                let next_board = self.next(action);
                if next_board.is_win() {
                    return action;
                } else if next_board.is_draw() {
                    return action;
                } else {
                    let val = -next_board._minimax_action(depth - 1);
                    if val == 1 {
                        return action;
                    }
                    if max_val < val {
                        max_val = val;
                        actions = vec![action];
                    } else if max_val == val {
                        actions.push(action);
                    }
                }
            }

            // println!("actions:{:#?}, val:{}", actions, max_val);
            return actions[rng.gen::<usize>() % actions.len()];
        }
    }

    fn to_string(&self) -> String {
        return format!("{},{}", self.black, self.white);
    }

    fn _minimax_action(&self, depth: u8) -> i8 {
        if depth == 0 {
            return 0;
        }
        let mut max_val = -2;
        for action in self.valid_actions() {
            let next_board = self.next(action);
            if next_board.is_win() {
                return 1;
            } else if next_board.is_draw() {
                return 0;
            }
            let val = -next_board._minimax_action(depth - 1);
            if max_val < val {
                max_val = val;
            }
        }
        return max_val;
    }

    pub fn is_black(&self) -> bool {
        match self.player {
            Player::Black => true,
            Player::White => false,
        }
    }
}

pub fn _is_win_board(bit: u64) -> bool {
    (bit & (bit >> 1) & (bit >> 2) & (bit >> 3) & 0x1111111111111111)
        | (bit & (bit >> 4) & (bit >> 8) & (bit >> 12) & 0x000f000f000f000f)
        | (bit & (bit >> 16) & (bit >> 32) & (bit >> 48) & 0x000000000000ffff)
        | (bit & (bit >> 5) & (bit >> 10) & (bit >> 15) & 0x0001000100010001)
        | (bit & (bit >> 3) & (bit >> 6) & (bit >> 9) & 0x0008000800080008)
        | (bit & (bit >> 17) & (bit >> 34) & (bit >> 51) & 0x1111)
        | (bit & (bit >> 15) & (bit >> 30) & (bit >> 45) & 0x8888)
        | (bit & (bit >> 20) & (bit >> 40) & (bit >> 60) & 0x000f)
        | (bit & (bit >> 12) & (bit >> 24) & (bit >> 36) & 0xf000)
        | (bit & (bit >> 21) & (bit >> 42) & (bit >> 63))
        | (bit & (bit >> 19) & (bit >> 38) & (bit >> 57) & 0x0008)
        | (bit & (bit >> 13) & (bit >> 26) & (bit >> 39) & 0x1000)
        | (bit & (bit >> 11) & (bit >> 22) & (bit >> 33) & 0x8000)
        > 0
}

pub fn get_random(board: &Board) -> u8 {
    let mut rng = rand::thread_rng();
    let actions = board.valid_actions();
    return actions[rng.gen::<usize>() % actions.len()];
}

pub fn pprint_board(board: &Board) {
    let mut s = String::new();
    for i in 0..4 {
        for j in 0..4 {
            for k in 0..4 {
                let idx = j * 16 + i * 4 + k;
                if (board.black >> idx) & 1 == 1 {
                    s += "O";
                } else if (board.white >> idx) & 1 == 1 {
                    s += "X";
                } else {
                    s += "-";
                }
            }
            s += " | ";
        }
        s += "\n"
    }
    print!("{}", s);
}

fn playout(board: &Board) -> f32 {
    let mut b = board.clone();
    let mut coef = 1.0;
    loop {
        let action = get_random(&b);
        b = b.next(action);
        if b.is_win() {
            return coef;
        } else if b.is_draw() {
            return 0.0;
        }
        coef *= -1.0;
    }
}

pub struct Node {
    board: Board,
    n: f32,
    w: f32,
    children: HashMap<u8, RefCell<Node>>,
}

// #[derive(Debug)]
pub struct Score {
    pub action: u8,
    pub score: f32,
    pub q: f32,
    pub na: f32,
    pub n: f32,
}

impl fmt::Debug for Score {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "action: {:>2}, score: {:>5.2}%({:>7.0}/{:>7.0}), Q: {:>5.3}",
            self.action,
            self.score * 100.0,
            self.na,
            self.n,
            self.q
        );
        Ok(())
    }
}

impl Node {
    pub fn new(board: Board) -> Self {
        return Node {
            board: board,
            children: HashMap::new(),
            n: 1f32,
            w: 0f32,
        };
    }

    pub fn search(&mut self, expand_n: usize, search_n: usize) -> Vec<Score> {
        if self.children.len() == 0 {
            self.expand();
        }
        for _ in 0..search_n {
            self.evaluate(expand_n);
        }

        let mut scores = Vec::new();
        for (action, node) in self.children.iter() {
            scores.push(Score {
                action: *action,
                score: node.borrow().n / self.n,
                q: node.borrow().w / node.borrow().n,
                na: node.borrow().n,
                n: self.n,
            });
            // println!("{}/{}", node.borrow().n, self.n);
        }
        return scores;
    }

    fn evaluate(&mut self, expand_n: usize) -> f32 {
        if self.board.is_win() {
            self.w += 1.0;
            self.n += 1.0;
            return 1.0;
        } else if self.board.is_draw() {
            self.n += 1.0;
            return 0.0;
        } else if self.children.len() == 0 {
            let value = playout(&self.board);
            self.w += value;
            self.n += 1.0;
            if self.n == expand_n as f32 {
                self.expand();
            }
            return value;
        } else {
            let next_node_action = {
                let children = &self.children;
                let mut best_action = 0;
                // (best_action, best_node) = &children[&0];
                let mut max_score = -2.0;
                for (action, node) in children.iter() {
                    let ucb = node.borrow().get_uct(self.n);
                    if ucb > max_score {
                        max_score = ucb;
                        best_action = *action;
                    }
                }
                best_action
            };
            let value = -self
                .children
                .get(&next_node_action)
                .unwrap()
                .borrow_mut()
                .evaluate(expand_n);
            self.w += value;
            self.n += 1.0;
            return value;
        }
    }

    fn expand(&mut self) {
        let mut nodes = HashMap::new();
        for action in self.board.valid_actions() {
            nodes.insert(action, RefCell::new(Node::new(self.board.next(action))));
        }
        self.children = nodes
    }

    fn get_uct(&self, N: f32) -> f32 {
        return self.w / self.n + (2.0 * N.ln() / self.n).sqrt();
    }
}

pub fn mcts_action(board: &Board, n: usize, ex_n: usize) -> u8 {
    let mut node = Node::new(board.clone());
    let scores = node.search(ex_n, n);
    // let mut max_action = 0;
    let mut max_actions = Vec::new();
    let mut max_score = -2.0;
    for score in scores {
        if score.score > max_score {
            max_score = score.score;
            // max_action = score.action;
            max_actions = vec![score.action];
        } else if score.score == max_score {
            max_actions.push(score.action);
        }
    }
    let mut rng = rand::thread_rng();
    return max_actions[rng.gen::<usize>() % max_actions.len()];
    // return max_action;
}

pub enum Agent {
    Human,
    Random,
    Minimax(u8),
    Mcts(usize, usize),
}

impl Agent {
    pub fn get_action(&self, board: &Board) -> u8 {
        match self {
            Agent::Human => {
                input! {
                    action: u8
                }
                action
            }
            Agent::Minimax(depth) => board.minimax_action(*depth),
            Agent::Mcts(expand_n, search_n) => mcts_action(board, *search_n, *expand_n),
            Agent::Random => get_random(board),
        }
    }

    pub fn name(&self) -> String {
        match self {
            Agent::Human => String::from("Human"),
            Agent::Random => String::from("Random"),
            Agent::Minimax(depth) => format!("Minimax:{}", depth),
            Agent::Mcts(ex, se) => format!("Mcts:{}/{}", se, ex),
        }
    }
}

pub fn play(a1: &Agent, a2: &Agent) -> (f32, f32) {
    let mut b = Board::new();
    loop {
        if b.is_black() {
            let action = a1.get_action(&b);
            b = b.next(action);
            if b.is_win() {
                return (1.0, 0.0);
            } else if b.is_draw() {
                return (0.5, 0.5);
            }
        } else {
            let action = a2.get_action(&b);
            b = b.next(action);
            if b.is_win() {
                return (0.0, 1.0);
            } else if b.is_draw() {
                return (0.5, 0.5);
            }
        }
    }
}

pub fn eval(a1: &Agent, a2: &Agent, n: usize) -> (f32, f32) {
    let mut score1 = 0.0;
    let mut score2 = 0.0;

    for i in 0..n {
        let (s1, s2) = play(a1, a2);
        score1 += s1;
        score2 += s2;
        // println!("game black: {}, s1:{}, s2:{}", i, s1, s2);
        let (s2, s1) = play(a2, a1);
        score1 += s1;
        score2 += s2;
        // println!("game white: {}, s1:{}, s2:{}", i, s1, s2);
    }
    return (score1 / (2 * n) as f32, score2 / (2 * n) as f32);
}
