use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

fn main() {
	let lines = fs::read_to_string("inputs/13-1").unwrap();

	// We go to the fold point, then subtract 2 from the next line and
	// 3 from the next, etc. until we reach the end. folding along y
	// we subtract from y. folding along y we subtract from x.
}
