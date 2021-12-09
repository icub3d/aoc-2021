use std::iter::Iterator;

#[derive(Clone, Debug)]
pub struct Permutation {
	v: Vec<usize>,
	c: Vec<usize>,
	i: usize,
	n: usize,
	first: bool,
}

impl Permutation {
	/// Create a new permutation iterator for an n-size vector.
	pub fn new(n: usize) -> Permutation {
		let mut v = Vec::new();
		let mut c = Vec::new();
		for x in 0..n {
			v.push(x);
			c.push(0);
		}
		Permutation {
			v,
			c,
			i: 0,
			n,
			first: true,
		}
	}
}

impl Iterator for Permutation {
	type Item = Vec<usize>;

	fn next(&mut self) -> Option<Vec<usize>> {
		// Heap's Algorithm (https://en.wikipedia.org/wiki/Heap%27s_algorithm)
		if self.first {
			// The initial vector is the first permutation.
			self.first = false;
			return Some(self.v.clone());
		}

		while self.i < self.n {
			if self.c[self.i] < self.i {
				if self.i % 2 == 0 {
					self.v.swap(0, self.i);
				} else {
					self.v.swap(self.c[self.i], self.i);
				}
				self.c[self.i] += 1;
				self.i = 0;
				return Some(self.v.clone());
			} else {
				self.c[self.i] = 0;
				self.i += 1;
			}
		}

		None
	}
}

#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		let result = 2 + 2;
		assert_eq!(result, 4);
	}
}
