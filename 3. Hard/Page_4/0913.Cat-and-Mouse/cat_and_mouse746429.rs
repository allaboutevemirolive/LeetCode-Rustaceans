// https://leetcode.com/problems/cat-and-mouse/solutions/746429/clean-rust-solution/
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Player {
    Cat,
    Mouse,
}

impl Player {
    pub fn opposite(&self) -> Player {
        match self {
            Player::Cat => Player::Mouse,
            Player::Mouse => Player::Cat,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
struct State {
    turn: Player,
    cat_pos: i32,
    mouse_pos: i32,
}

impl State {
    const WINNING_MOUSE_POS: i32 = 0;

    pub fn new(turn: Player, cat_pos: i32, mouse_pos: i32) -> State {
        State {
            turn,
            cat_pos,
            mouse_pos,
        }
    }

    pub fn winner(&self) -> Option<Player> {
        if self.mouse_pos == State::WINNING_MOUSE_POS || self.cat_pos == State::WINNING_MOUSE_POS {
            // If either the mouse or cat falls into the hole, the mouse wins.
            Some(Player::Mouse)
        } else if self.mouse_pos == self.cat_pos {
            // If the cat catches the mouse, the cat wins.
            Some(Player::Cat)
        } else {
            // Otherwise there isn't a winner yet.
            None
        }
    }
}

pub fn cat_mouse_game(pos_graph: Vec<Vec<i32>>) -> i32 {
    let mut graph: HashMap<State, HashSet<State>> = HashMap::new();
    let mut rgraph: HashMap<State, HashSet<State>> = HashMap::new();
    let mut states = HashSet::new();
    let mut state_winner: HashMap<State, Player> = HashMap::new();
    for (pos_one, pos_adjs) in pos_graph.iter().enumerate() {
        for pos_adj in pos_adjs {
            for turn in &[Player::Mouse, Player::Cat] {
                for cnst_pos in 0..pos_graph.len() {
                    let (cat_pos, new_cat_pos, mouse_pos, new_mouse_pos) = match *turn {
                        Player::Cat => (pos_one as i32, *pos_adj, cnst_pos as i32, cnst_pos as i32),
                        Player::Mouse => {
                            (cnst_pos as i32, cnst_pos as i32, pos_one as i32, *pos_adj)
                        }
                    };
                    let from = State::new(*turn, cat_pos, mouse_pos);
                    let to = State::new(turn.opposite(), new_cat_pos, new_mouse_pos);
                    graph.entry(from).or_insert(HashSet::new()).insert(to);
                    rgraph.entry(to).or_insert(HashSet::new()).insert(from);
                    for state in vec![from, to] {
                        if let Some(winning_player) = state.winner() {
                            state_winner.insert(state, winning_player);
                        }
                        states.insert(state);
                    }
                }
            }
        }
    }

    let mut degree_left = HashMap::new();
    let mut stack = Vec::new();
    for state in states {
        if let Some(adjs) = graph.get(&state) {
            // The degree is the number of children left. Once all children are
            // complete, the winner of this state can be determined
            degree_left.insert(state, adjs.len() as i32);
        } else {
            degree_left.insert(state, 0);
        }

        if let Some(_) = state_winner.get(&state) {
            degree_left.insert(state, 0);
        }

        if degree_left[&state] == 0 {
            stack.push(state);
        }
    }

    while let Some(state) = stack.pop() {
        let winner = state_winner[&state];
        for &parent in rgraph.get(&state).unwrap_or(&HashSet::new()) {
            if state_winner.contains_key(&parent) {
                continue;
            }

            // If it was the winner's turn last turn, then they can make the winning move.
            if winner == parent.turn {
                state_winner.insert(parent, winner);
                stack.push(parent);
                continue;
            }

            // Otherwise, decrease the `degree_left` count and if the degree is 0, this parent
            // has no winning moves, and therefore we can set the winner.
            // Safe unwrap as all states were put into `degree_left`.
            *degree_left.get_mut(&parent).unwrap() -= 1;
            if degree_left[&parent] == 0 {
                state_winner.insert(parent, parent.turn.opposite());
                stack.push(parent);
            }
        }
    }

    match state_winner.get(&State::new(Player::Mouse, 2, 1)) {
        None => 0,
        Some(Player::Mouse) => 1,
        Some(Player::Cat) => 2,
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_cat_mouse_simple() {
        assert_eq!(
            cat_mouse_game(vec![
                vec![2, 5],
                vec![3],
                vec![0, 4, 5],
                vec![1, 4, 5],
                vec![2, 3],
                vec![0, 2, 3],
            ]),
            0
        );
    }

    #[test]
    fn test_cat_mouse_simple2() {
        /*
            4 - 3 - 0 - 2 - 1
        */
        assert_eq!(
            cat_mouse_game(vec![vec![2, 3], vec![2], vec![0, 1], vec![0, 4], vec![3]]),
            2
        );
    }

    #[test]
    fn test_cat_mouse_simple3() {
        /*
            0 - 2 - 1
        */
        assert_eq!(cat_mouse_game(vec![vec![2], vec![2], vec![0, 1]]), 2);
    }

    #[test]
    fn test_cat_mouse_cycle() {
        assert_eq!(
            cat_mouse_game(vec![
                vec![6],
                vec![4],
                vec![9],
                vec![5],
                vec![1, 5],
                vec![3, 4, 6],
                vec![0, 5, 10],
                vec![8, 9, 10],
                vec![7],
                vec![2, 7],
                vec![6, 7]
            ]),
            1
        );
    }
}