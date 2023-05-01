// https://leetcode.com/problems/game-of-life/solutions/1940479/rust-simple-in-place/
fn game_of_life(board: &mut Vec<Vec<i32>>) {
	const NEIGHBOURS: &[(isize, isize)] = &[
		(-1, -1),
		(0, -1),
		(1, -1),

		(-1, 0),
		(1, 0),

		(-1, 1),
		(0, 1),
		(1, 1),
	];

	let m = board.len() as isize;
	let n = board[0].len() as isize;

	for y in 0..m {
		for x in 0..n {
			let neighbours = NEIGHBOURS.iter()
				.map(|(xoffset, yoffset)| (x + xoffset, y + yoffset))
				.filter(|&(nx, ny)| nx >= 0 && ny >= 0 && nx < n && ny < m)
				.map(|(nx, ny)| board[ny as usize][nx as usize] & 1)
				.sum();
			let (x, y) = (x as usize, y as usize);
			board[y][x] |= if board[y][x] == 1 {
				match neighbours {
					2..=3 => 1,
					_ => 0,
				}
			} else {
				if neighbours == 3 {
					1
				} else {
					0
				}
			} << 1;
		}
	}
	board.iter_mut()
		.for_each(|row| {
			row.iter_mut().for_each(|value| { *value >>= 1; })
		});
}