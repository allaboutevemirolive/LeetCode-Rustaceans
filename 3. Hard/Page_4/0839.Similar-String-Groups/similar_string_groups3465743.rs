// https://leetcode.com/problems/similar-string-groups/solutions/3465743/rust-dfs-connected-components/
impl Solution {
    fn check(&self, s1: &String, s2: &String) -> bool {
        let diff = s1
            .chars()
            .zip(s2.chars())
            .map(|(c1, c2)| c1 == c2)
            .filter(|x| !x)
            .count();
        diff == 2 || diff == 0
    }

    fn dfs(
        &self,
        src_group: usize,
        group: &mut Vec<i32>,
        groups: &mut Vec<Vec<usize>>,
        strs: &Vec<String>,
    ) {
        for (ind, s) in strs.iter().enumerate() {
            if group[ind] == -1 {
                let is_same_group = groups[src_group]
                    .iter()
                    .map(|ind| &strs[*ind])
                    .any(|s_in_group| self.check(s_in_group, s));
                if is_same_group {
                    groups[src_group].push(ind);
                    group[ind] = src_group as i32;

                    self.dfs(src_group, group, groups, strs);
                }
            }
        }
    }
    
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let mut group: Vec<i32> = vec![-1; strs.len()];
        let mut groups: Vec<Vec<usize>> = vec![];
        let mut cnt = 0;

        let sol = Solution {};

        for (ind, s) in strs.iter().enumerate() {
            if group[ind] == -1 {
                cnt += 1;
                group[ind] = groups.len() as i32;
                groups.push(vec![]);
                groups[group[ind] as usize].push(ind);

                sol.dfs(group[ind] as usize, &mut group, &mut groups, &strs);
            }
        }

        groups.len() as i32
    }
}