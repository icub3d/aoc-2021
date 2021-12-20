use lazy_static::lazy_static;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Debug)]
struct Transform {
	x: Point,
	y: Point,
	z: Point,
}

impl Transform {
	fn new(x: Point, y: Point, z: Point) -> Self {
		Self { x, y, z }
	}

	fn transform(&self, scanner: &Scanner) -> Scanner {
		scanner
			.iter()
			.map(|beacon| {
				Point::new(
					self.x.x * beacon.x + self.x.y * beacon.y + self.x.z + beacon.z,
					self.y.x * beacon.x + self.y.y * beacon.y + self.y.z + beacon.z,
					self.z.x * beacon.x + self.z.y * beacon.y + self.z.z + beacon.z,
				)
			})
			.collect()
	}
}

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
struct Point {
	x: isize,
	y: isize,
	z: isize,
}

impl Point {
	fn new(x: isize, y: isize, z: isize) -> Self {
		Self { x, y, z }
	}

	fn add(&self, other: &Point) -> Point {
		Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}

	fn subtract(&self, other: &Point) -> Point {
		Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
	fn manhattan_distance(&self, other: &Point) -> isize {
		let x = self.x - other.x;
		let y = self.y - other.y;
		let z = self.z - other.z;
		x.abs() + y.abs() + z.abs()
	}
}

type Scanner = HashSet<Point>;

fn attempt_match(first: &Scanner, second: &Scanner) -> Option<(Scanner, Point)> {
	for transformer in TRANSFORMERS.iter() {
		let transformed = transformer.transform(second);
		for left in first.iter() {
			for right in transformed.iter() {
				let position = left.subtract(right);
				println!("{:?} {:?}", position, transformer);
				let translated = transformed.iter().map(|p| p.add(&position)).collect();
				if first.intersection(&translated).count() >= 12 {
					return Some((translated, position));
				}
			}
		}
	}
	None
}

fn main() {
	let input = include_str!("../input");
	let scanners = input
		.split("\n\n")
		.map(|l| {
			l.lines()
				.skip(1)
				.map(|l| {
					let pp = l
						.split(',')
						.map(|s| s.parse::<isize>().unwrap())
						.collect::<Vec<isize>>();
					Point::new(pp[0], pp[1], pp[2])
				})
				.collect::<Scanner>()
		})
		.collect::<Vec<Scanner>>();

	// Part 1 - assume the first is oriented correctly and then try to
	// orient the rest. Keep going until we've oriented them all.
	let mut oriented: HashMap<usize, Scanner> = HashMap::new();
	oriented.insert(0, scanners[0].clone());
	let mut completed = HashSet::new();

	// Part 2 - Manhattan distances.
	let mut positions = Vec::new();
	positions.push(Point::new(0, 0, 0));
	loop {
		for i in 0..scanners.len() {
			// If this one is done, we don't need to check again. We
			// also don't want to try this one if we haven't found
			// it's orientation yet.
			if completed.contains(&i.clone()) || !oriented.contains_key(&i.clone()) {
				continue;
			}

			for (j, second) in scanners.iter().enumerate() {
				// Don't want to check against ourselves or against one we
				// already have oriented.
				if i == j || oriented.contains_key(&j.clone()) {
					continue;
				}

				// Try to match and if we found one, update our information.
				let found = attempt_match(oriented.get(&i).unwrap(), second);
				std::process::exit(1);
				if let Some((found, position)) = found {
					println!("found: {} {} {}", i, j, completed.len());
					oriented.insert(j, found);
					positions.push(position)
				}
			}

			completed.insert(i.clone());
		}

		// Once we've done them all, we are finished.
		if completed.len() == scanners.len() {
			break;
		}
	}

	// Now that they are all oriented, we just union all the sets and
	// that should give us part one.
	let p1 = oriented.values().cloned().fold(Scanner::new(), |acc, s| {
		acc.union(&s).cloned().collect::<Scanner>()
	});
	println!("p1: {}", p1.len());
}

lazy_static! {
	static ref TRANSFORMERS: Vec<Transform> = vec![
		Transform::new(
			Point::new(-1, 0, 0),
			Point::new(0, -1, 0),
			Point::new(0, 0, 1)
		),
		Transform::new(
			Point::new(-1, 0, 0),
			Point::new(0, 0, -1),
			Point::new(0, -1, 0)
		),
		Transform::new(
			Point::new(-1, 0, 0),
			Point::new(0, 0, 1),
			Point::new(0, 1, 0)
		),
		Transform::new(
			Point::new(-1, 0, 0),
			Point::new(0, 1, 0),
			Point::new(0, 0, -1)
		),
		Transform::new(
			Point::new(0, -1, 0),
			Point::new(-1, 0, 0),
			Point::new(0, 0, -1)
		),
		Transform::new(
			Point::new(0, -1, 0),
			Point::new(0, 0, -1),
			Point::new(1, 0, 0)
		),
		Transform::new(
			Point::new(0, -1, 0),
			Point::new(0, 0, 1),
			Point::new(-1, 0, 0)
		),
		Transform::new(
			Point::new(0, -1, 0),
			Point::new(1, 0, 0),
			Point::new(0, 0, 1)
		),
		Transform::new(
			Point::new(0, 0, -1),
			Point::new(-1, 0, 0),
			Point::new(0, 1, 0)
		),
		Transform::new(
			Point::new(0, 0, -1),
			Point::new(0, -1, 0),
			Point::new(-1, 0, 0)
		),
		Transform::new(
			Point::new(0, 0, -1),
			Point::new(0, 1, 0),
			Point::new(1, 0, 0)
		),
		Transform::new(
			Point::new(0, 0, -1),
			Point::new(1, 0, 0),
			Point::new(0, -1, 0)
		),
		Transform::new(
			Point::new(0, 0, 1),
			Point::new(-1, 0, 0),
			Point::new(0, -1, 0)
		),
		Transform::new(
			Point::new(0, 0, 1),
			Point::new(0, -1, 0),
			Point::new(1, 0, 0)
		),
		Transform::new(
			Point::new(0, 0, 1),
			Point::new(0, 1, 0),
			Point::new(-1, 0, 0)
		),
		Transform::new(
			Point::new(0, 0, 1),
			Point::new(1, 0, 0),
			Point::new(0, 1, 0)
		),
		Transform::new(
			Point::new(0, 1, 0),
			Point::new(-1, 0, 0),
			Point::new(0, 0, 1)
		),
		Transform::new(
			Point::new(0, 1, 0),
			Point::new(0, 0, -1),
			Point::new(-1, 0, 0)
		),
		Transform::new(
			Point::new(0, 1, 0),
			Point::new(0, 0, 1),
			Point::new(1, 0, 0)
		),
		Transform::new(
			Point::new(0, 1, 0),
			Point::new(1, 0, 0),
			Point::new(0, 0, -1)
		),
		Transform::new(
			Point::new(1, 0, 0),
			Point::new(0, -1, 0),
			Point::new(0, 0, -1)
		),
		Transform::new(
			Point::new(1, 0, 0),
			Point::new(0, 0, -1),
			Point::new(0, 1, 0)
		),
		Transform::new(
			Point::new(1, 0, 0),
			Point::new(0, 0, 1),
			Point::new(0, -1, 0)
		),
		Transform::new(
			Point::new(1, 0, 0),
			Point::new(0, 1, 0),
			Point::new(0, 0, 1)
		),
	];
}
