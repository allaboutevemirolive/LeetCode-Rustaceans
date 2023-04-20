// https://leetcode.com/problems/contain-virus/solutions/578511/clear-rust-solution-with-simple-explains/
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

type Point = (usize, usize);

#[derive(Eq, Debug)]
struct InfluencedRecord {
    pub need_walls: i32,
    pub influenced_area: HashSet<Point>,
    pub virus_area: HashSet<Point>,
}

impl Ord for InfluencedRecord {
    fn cmp(&self, other: &Self) -> Ordering {
        self.influenced_area.len().cmp(&other.influenced_area.len())
    }
}

impl PartialOrd for InfluencedRecord {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for InfluencedRecord {
    fn eq(&self, other: &Self) -> bool {
        self.influenced_area.len() == other.influenced_area.len()
    }
}

impl Solution {
    pub fn contain_virus(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut safe_areas = HashSet::new();
        //record the  need walls
        let mut result = 0;
        let row_num = grid.len();
        let col_num = grid[0].len();
        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            safe_areas: &mut HashSet<Point>,
            result: &mut i32,
            row_num: usize,
            col_num: usize,
        ) {
            //record all virus grid,virus area and influenced area;
            let mut virus = HashSet::new();
            let mut influenced_records = BinaryHeap::new();
            fn find_virus_and_influenced_area(
                virus_area: &mut HashSet<Point>,
                influenced_area: &mut HashSet<Point>,
                virus: &mut HashSet<Point>,
                safe_areas: &mut HashSet<Point>,
                need_walls: &mut i32,
                grid: &Vec<Vec<i32>>,
                point: Point,
                row_num: usize,
                col_num: usize,
            ) {
                // top right down left
                let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
                virus_area.insert(point);
                virus.insert(point);
                // top right down left
                dirs.iter().for_each(|&(x, y)| {
                    let new_x = point.0 as i32 + x;
                    let new_y = point.1 as i32 + y;
                    if new_x >= 0
                        && new_x < (row_num as i32)
                        && new_y >= 0
                        && new_y < (col_num as i32)
                    {
                        let new_point = (new_x as usize, new_y as usize);
                        let grid_val = &grid[new_x as usize][new_y as usize];
                        if grid_val == &0 {
                            influenced_area.insert(new_point);
                            *need_walls += 1;
                            return;
                        }
                        if !virus.contains(&new_point) && !safe_areas.contains(&new_point) {
                            find_virus_and_influenced_area(
                                virus_area,
                                influenced_area,
                                virus,
                                safe_areas,
                                need_walls,
                                &grid,
                                new_point,
                                row_num,
                                col_num,
                            );
                        }
                    }
                });
            }
            for i in 0..row_num {
                for j in 0..col_num {
                    let grid_val = grid[i][j];
                    if grid_val == 0 {
                        continue;
                    }
                    let point = (i, j);
                    if !virus.contains(&point) && !safe_areas.contains(&point) {
                        let mut virus_area = HashSet::new();
                        let mut influenced_area = HashSet::new();
                        let mut need_walls = 0;
                        //find contiguous virus and influenced_area;
                        find_virus_and_influenced_area(
                            &mut virus_area,
                            &mut influenced_area,
                            &mut virus,
                            safe_areas,
                            &mut need_walls,
                            grid.as_ref(),
                            point,
                            row_num,
                            col_num,
                        );
                        influenced_records.push(InfluencedRecord {
                            virus_area,
                            influenced_area,
                            need_walls,
                        });
                    }
                }
            }
            let influenced_record = influenced_records.pop();
            if influenced_record.is_none() {
                return;
            }
            let InfluencedRecord {
                mut virus_area,
                influenced_area: _,
                need_walls,
            } = influenced_record.unwrap();
            //build walls;
            *result += need_walls;
            //add safe areas;
            safe_areas.extend(virus_area.drain());
            influenced_records.iter().for_each(
                |InfluencedRecord {
                     influenced_area,
                     virus_area: _,
                     need_walls: _,
                 }| {
                    //infect areas;
                    influenced_area.iter().for_each(|&(x, y)| {
                        grid[x][y] = 1;
                    });
                },
            );
            //continue;
            dfs(grid, safe_areas, result, row_num, col_num);
        }
        dfs(&mut grid, &mut safe_areas, &mut result, row_num, col_num);
        result
    }
}