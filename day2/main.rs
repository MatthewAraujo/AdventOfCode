fn part1(input: &str) -> u32 {
	input.lines().filter_map(|line| {
		let (game, rest) = line.split_once(": ")?;
		let id = game.strip_prefix("Game ")?.parse::<u32>().ok()?;
	
		let is_valid = rest.split("; ").all(|set| {
			set.split(", ").all(|cube| {
				let (digit, color) = cube.split_once(" ").unwrap();
				let digit = digit.parse::<u32>().unwrap();
	
				match color {
					"blue" => digit <= 14,
					"green" => digit <= 13,
					"red" => digit <= 12,
					_ => false
				}
			})
		});
		if is_valid { Some(id) } else { None }
	}).sum::<u32>()    
}

fn part2(input: &str) -> u32 {
	input.lines().map(|line| {
		let (_, rest) = line.split_once(": ").unwrap();
	
		let mut blue = 0;
		let mut green = 0;
		let mut red = 0;
		rest.split("; ").for_each(|set| {
			set.split(", ").for_each(|cube| {
				let (digit, color) = cube.split_once(" ").unwrap();
				let digit = digit.parse::<u32>().unwrap();
	
				match color {
					"blue" => blue = blue.max(digit),
					"green" => green = green.max(digit),
					"red" => red = red.max(digit),
					_ => {},
				}
			});
		});
		blue * green * red
	}).sum::<u32>()    
}
fn main() {
	let input = include_str!("input2.txt");
	println!("{}, {}", part1(input), part2(input));
}
