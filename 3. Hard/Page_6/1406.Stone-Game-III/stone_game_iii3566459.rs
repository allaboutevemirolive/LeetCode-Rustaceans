// https://leetcode.com/problems/stone-game-iii/solutions/3566459/rust-reverse-sliding-window-one-pass/
enum States {
    Entry,
    Op(OpStates, i32),
}

impl States {
    pub fn accept(self, token: i32) -> (States, i32) {
        match self {
            Self::Entry => {
                (Self::Op(
                    OpStates::One { val: token },
                    token,
                ), token)
            },
            Self::Op(opstate, cum) => {
                let (opstate_next, res) = opstate.accept(token, cum);

                (Self::Op(
                    opstate_next,
                    cum + token
                ), res)
            }
        }
    }
}

enum OpStates {
    One {
        val: i32,
    },
    Two {
        vals: [i32; 2],
        res: i32,
    },
    Three {
        vals: [i32; 3],
        ress: [i32; 3],
    }
}

impl OpStates {
    pub fn accept(self, token: i32, cum: i32) -> (Self, i32) {
        match self {
            Self::One { val } => {
                let res_next = token.max(token + val);

                (Self::Two {
                    vals: [token, val],
                    res: res_next,
                }, res_next)
            },
            Self::Two { vals, res } => {
                let res_next = token + cum - res; // pick 1
                let res_next = res_next.max(token + vals[0]); // pick 2, left 1
                let res_next = res_next.max(token + vals[0] + vals[1]); // pick all 3

                (Self::Three {
                    vals: [token, vals[0], vals[1]],
                    ress: [res_next, res, vals[1]],
                }, res_next)
            },
            Self::Three { vals, ress } => {
                let res_next = token + cum - ress[0];
                let res_next = res_next.max(token + cum - ress[1]);
                let res_next = res_next.max(token + cum - ress[2]);

                (Self::Three {
                    vals: [token, vals[0], vals[1]],
                    ress: [res_next, ress[0], ress[1]],
                }, res_next)
            }
        }
    }
}

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let mut state = States::Entry;

        let mut val_opt: Option<i32> = None;
        for e in stone_value.into_iter().rev() {
            let (next_state, val) = state.accept(e);
            val_opt = Some(val);
            state = next_state;
        }

        let max_val = val_opt.unwrap();
        let total_score = if let States::Op(_, val) = state {
            val
        } else {
            0
        };

        if max_val * 2 < total_score {
            "Bob".to_string()
        } else if max_val * 2 > total_score {
            "Alice".to_string()
        } else {
            "Tie".to_string()
        }
    }
}