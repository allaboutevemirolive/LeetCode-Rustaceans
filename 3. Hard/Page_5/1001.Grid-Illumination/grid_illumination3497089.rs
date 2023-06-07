// https://leetcode.com/problems/grid-illumination/solutions/3497089/rust-b-tree-solution/
use std::collections::BTreeSet;
use std::ops::{Bound, RangeBounds};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct Position {
    row: i32,
    col: i32,
}

impl Position {
    pub const MIN: Self = Self{row: i32::MIN, col: i32::MIN};
    pub const MAX: Self = Self{row: i32::MAX, col: i32::MAX};

    #[inline]
    pub fn from_item(item: Vec<i32>) -> Self {
        match item[..] {
            [row, col] => Position{row, col},
            _ => panic!(),
        }
    }
    #[inline]
    pub fn row(self: &Self) -> i32 { self.row }
    #[inline]
    pub fn col(self: &Self) -> i32 { self.col }
    #[inline]
    pub fn mdiag(self: &Self) -> i32 { self.row - self.col }
    #[inline]
    pub fn sdiag(self: &Self) -> i32 { self.row + self.col }
    #[inline]
    pub fn adjacent(self: &Self) -> impl Iterator<Item=Position> + '_ {
        (self.row-1..=self.row+1).map(move |row| {
            (self.col-1..=self.col+1).map(move |col| {
                Position{row, col}
            })
        }).flatten()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
struct IdxPosition {
    index: i32,
    position: Position,
}

impl IdxPosition {
    #[inline]
    pub fn by_row(lamp: Position) -> Self {
        return Self{position: lamp, index: lamp.row()};
    }
    #[inline]
    pub fn by_col(lamp: Position) -> Self {
        return Self{position: lamp, index: lamp.col()};
    }
    #[inline]
    pub fn by_mdiag(lamp: Position) -> Self {
        return Self{position: lamp, index: lamp.mdiag()};
    }
    #[inline]
    pub fn by_sdiag(lamp: Position) -> Self {
        return Self{position: lamp, index: lamp.sdiag()};
    }
    #[inline]
    pub fn as_bound(self: Self) -> impl RangeBounds<IdxPosition> {
        ( Bound::Included(Self{index: self.index, position: Position::MIN}),
            Bound::Included(Self{index: self.index, position: Position::MAX}) )
    }
}

#[derive(Debug)]
struct Index {
    row_index: BTreeSet<IdxPosition>,
    col_index: BTreeSet<IdxPosition>,
    mdiag_index: BTreeSet<IdxPosition>,
    sdiag_index: BTreeSet<IdxPosition>,
}

impl Index {
    pub fn new() -> Self {
        Self {
            row_index: BTreeSet::new(),
            col_index: BTreeSet::new(),
            mdiag_index: BTreeSet::new(),
            sdiag_index: BTreeSet::new(),
        }
    }
    pub fn insert(self: &mut Self, lamp: Position) -> () {
        self.row_index.insert(IdxPosition::by_row(lamp));
        self.col_index.insert(IdxPosition::by_col(lamp));
        self.mdiag_index.insert(IdxPosition::by_mdiag(lamp));
        self.sdiag_index.insert(IdxPosition::by_sdiag(lamp));
    }
    pub fn is_illuminated(self: &Self, pos: Position) -> bool {
        self.row_index.range(IdxPosition::by_row(pos).as_bound()).count() > 0 ||
        self.col_index.range(IdxPosition::by_col(pos).as_bound()).count() > 0 ||
        self.mdiag_index.range(IdxPosition::by_mdiag(pos).as_bound()).count() > 0 ||
        self.sdiag_index.range(IdxPosition::by_sdiag(pos).as_bound()).count() > 0
    }
    pub fn remove(self: &mut Self, lamp: Position) -> bool {
        self.row_index.remove(&IdxPosition::by_row(lamp)) &&
        self.col_index.remove(&IdxPosition::by_col(lamp)) &&
        self.mdiag_index.remove(&IdxPosition::by_mdiag(lamp)) &&
        self.sdiag_index.remove(&IdxPosition::by_sdiag(lamp))
    }
}

impl Solution {
    pub fn grid_illumination(n: i32, lamps: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut index = Index::new();
        let mut answer: Vec<i32> = Vec::new();
        for lamp_item in lamps {
            index.insert(Position::from_item(lamp_item));
        }
        for query_item in queries {
            let position = Position::from_item(query_item);
            answer.push(if index.is_illuminated(position) { 1 } else { 0 });
            for adjacent in position.adjacent() {
                index.remove(adjacent);
            }
        }
        return answer;
    }
}
