// https://leetcode.com/problems/guess-the-word/solutions/1402198/rust-best-neighbour-minimax-solution/
fn levenstein(a: &str, b: &str) -> u8 {
    if a.len() != 6 || b.len() != 6 {
        panic!("Word length is specified to be 6!");
    }
    
    let mut result: u8 = 0;
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    for i in 0..6 {
        if a_chars[i] != b_chars[i] {
            result += 1;
        }
    }
    result
}

fn get_best_neighbour<'a>(words: &'a HashSet<&String>) -> &'a String {
    let mut best_neighbour: Option<&String> = None;
    let mut best_distance_agg: u64 = u64::MAX;
    
    for neighbour in words.iter() {
        let mut distance_agg: u64 = words.iter().map(|x| (levenstein(x, neighbour) as u64).pow(2)).sum();
        if distance_agg < best_distance_agg {
            best_distance_agg = distance_agg;
            best_neighbour = Some(neighbour);
        }
    }
    
    best_neighbour.unwrap()
}

impl Solution {
    pub fn find_secret_word(words: Vec<String>, master: &Master) {
        let mut words_set: HashSet<&String> = words.iter().collect();
        
        let mut guess = (*get_best_neighbour(&words_set)).clone();
        while words_set.len() > 1 {
            words_set.remove(&guess);
            println!("words_set.len(): {:?} guess: {:?}", words_set.len(), guess);
            
            let master_distance: u8 = (6 - master.guess(guess.clone())) as u8;
            println!("master_distance: {:?}", master_distance);
            if master_distance == 0 {
                return;
            } else {
                words_set.retain(|x| levenstein(x, &guess) == master_distance);
                guess = (*get_best_neighbour(&words_set)).clone();
            }
        }
        master.guess((*words_set.into_iter().nth(0).unwrap()).clone());
    }
}