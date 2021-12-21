use std::collections::HashMap;
use std::collections::HashSet;
use std::ops::{Add, Sub};

use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
struct Point {
	x: isize,
	y: isize,
	z: isize,
}

impl Add for Point {
	type Output = Self;
	fn add(self, other: Self) -> Self {
		Self::new(self.x + other.x, self.y + other.y, self.z + other.z)
	}
}

impl Sub for Point {
	type Output = Self;
	fn sub(self, other: Self) -> Self {
		Self::new(self.x - other.x, self.y - other.y, self.z - other.z)
	}
}

impl Point {
	fn new(x: isize, y: isize, z: isize) -> Self {
		Self { x, y, z }
	}

	// Based on the axis of rotation change the values.
	fn rotate(&self, axis: &Self) -> Self {
		let mut rotated = self.clone();
		// x-axis rotation
		for _ in 0..axis.x {
			let original_z = rotated.z;
			rotated.z = rotated.y;
			rotated.y = -original_z;
		}

		// y-axis rotation
		for _ in 0..axis.y {
			let original_z = rotated.z;
			rotated.z = -rotated.x;
			rotated.x = original_z;
		}

		// z-axis rotation
		for _ in 0..axis.z {
			let original_y = rotated.y;
			rotated.y = rotated.x;
			rotated.x = -original_y;
		}

		rotated
	}

	fn manhattan_distance(&self, other: &Point) -> isize {
		let x = self.x - other.x;
		let y = self.y - other.y;
		let z = self.z - other.z;
		x.abs() + y.abs() + z.abs()
	}
}

fn transforms() -> Vec<Point> {
	let mut tt = Vec::new();

	// I was doing all of these but it turns out many of them are
	// similar, so we keep a list of rotations we've done and if we've
	// already done it, we don't need to add it.
	let mut known = HashSet::new();
	for x in 0..=3 {
		for y in 0..=3 {
			for z in 0..=3 {
				let transform = Point::new(x, y, z);
				let rotations = (
					Point::new(1, 0, 0).rotate(&transform),
					Point::new(0, 1, 0).rotate(&transform),
					Point::new(0, 0, 1).rotate(&transform),
				);
				if known.contains(&rotations) {
					continue;
				}
				known.insert(rotations);
				tt.push(Point::new(x, y, z));
			}
		}
	}
	tt
}

type Scanner = HashSet<Point>;

fn attempt_match(first: &Scanner, second: &Scanner) -> Option<(Scanner, Point)> {
	for transformer in transforms().iter() {
		// Transform each of the second scanner's points.
		let transformed = second
			.iter()
			.map(|p| p.rotate(transformer))
			.collect::<Vec<Point>>();

		// For each of the points, see if we can find a match. We can
		// calculate the position of the scanners relative to each
		// other based on the point, so we can translate all of the
		// second position and see how many line up.
		for (left, right) in first.iter().cartesian_product(transformed.iter()) {
			let position = *left - *right;
			let translated = transformed.iter().map(|p| *p + position).collect();
			if first.intersection(&translated).count() >= 12 {
				return Some((translated, position));
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

	// We've kept all the distances, so we just need to find the ones
	// that are the furthest apart.
	let p2 = positions
		.clone()
		.iter()
		.cartesian_product(positions.iter())
		.map(|(i, j)| i.manhattan_distance(j))
		.max();
	println!("p2: {:?}", p2);
}
