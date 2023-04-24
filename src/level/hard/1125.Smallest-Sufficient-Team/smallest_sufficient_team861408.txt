// https://leetcode.com/problems/smallest-sufficient-team/solutions/861408/rust-translated/
impl Solution {
    pub fn smallest_sufficient_team(req_skills: Vec<String>, people: Vec<Vec<String>>) -> Vec<i32> {
        use std::collections::{HashMap, HashSet};
        use std::iter::FromIterator;
        fn dfs(
            skills_map: &HashMap<String, Vec<i32>>,
            req_skills: &Vec<String>,
            max_size: &mut usize,
            index: i32,
            set: &mut HashSet<i32>,
            res: &mut HashSet<i32>,
        ) {
            if set.len() >= *max_size {
                return;
            }
            if index == req_skills.len() as i32 {
                *max_size = set.len();
                res.clear();
                for &e in set.iter() {
                    res.insert(e);
                }
                return;
            }
            for &person in skills_map.get(&req_skills[index as usize]).unwrap() {
                let is_new = !set.contains(&person);
                set.insert(person);
                dfs(skills_map, req_skills, max_size, index + 1, set, res);
                if is_new {
                    set.remove(&person);
                }
            }
        }

        let mut max_size = people.len() + 1;
        let mut res_team = HashSet::<i32>::new();
        let mut skills_map = HashMap::<String, Vec<i32>>::new();
        for i in 0..people.len() {
            for skill in &people[i] {
                skills_map
                    .entry(skill.to_owned())
                    .or_default()
                    .push(i as i32);
            }
        }
        dfs(
            &skills_map,
            &req_skills,
            &mut max_size,
            0,
            &mut HashSet::new(),
            &mut res_team,
        );
        Vec::from_iter(res_team.into_iter())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smallest_sufficient_team() {
        assert_eq!(
            Solution::smallest_sufficient_team(
                vec!["java".to_owned(), "nodejs".to_owned(), "reactjs".to_owned()],
                vec![
                    vec!["java".to_owned()],
                    vec!["nodejs".to_owned()],
                    vec!["nodejs".to_owned(), "reactjs".to_owned()]
                ]
            ),
            vec![0, 2]
        );
    }

    #[test]
    fn test_smallest_sufficient_team_02() {
        assert_eq!(
            Solution::smallest_sufficient_team(
                vec![
                    "algorithms".to_owned(),
                    "math".to_owned(),
                    "java".to_owned(),
                    "reactjs".to_owned(),
                    "csharp".to_owned(),
                    "aws".to_owned()
                ],
                vec![
                    vec![
                        "algorithms".to_owned(),
                        "math".to_owned(),
                        "java".to_owned()
                    ],
                    vec![
                        "algorithms".to_owned(),
                        "math".to_owned(),
                        "reactjs".to_owned()
                    ],
                    vec!["java".to_owned(), "csharp".to_owned(), "aws".to_owned()],
                    vec!["reactjs".to_owned(), "csharp".to_owned()],
                    vec!["csharp".to_owned(), "math".to_owned()],
                    vec!["aws".to_owned(), "java".to_owned()]
                ]
            ),
            vec![1, 2]
        );
    }
}