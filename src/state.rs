use std::fmt::{Display, Formatter, Result};
use colored::Colorize;

/// A struct capturing the full state of the brainfuck data structure.
pub struct BFState {
	/// Vector containing the cell values on the band. Automatically extended to the right as required.
	data: Vec<u8>,
	
	/// Pointer to the current cell.
	idx: usize,
}

impl BFState {
	
	/// Initialize a new BFState with 30,000 cells containing 0u8 and initial pointer location at cell 15,000.
	pub fn new() -> BFState {
		let data = vec![0u8; 30_000];
		let idx = 15_000;
		
		BFState {data, idx}
	}
	
	
	/// Move the pointer up one cell.
	/// If the end of the data storage is reached, the data is extended by 5,000 further cells containing 0u8.
	pub fn up(&mut self) {
		if self.idx + 1 > self.data.len() {
			self.data.append(&mut vec![0u8; 5_000]);  // Extend data if end is reached
		}
		self.idx += 1;
	}
	
	
	/// Move the pointer down one cell. If the left edge of the data is reached, panic.
	pub fn down(&mut self) {
		self.idx -= 1;
	}
	
	
	/// Increment the value at the pointer.
	/// Wraps around from 255 to 0.
	pub fn incr(&mut self) {
		self.data[self.idx] = self.data[self.idx].wrapping_add(1);
	}
	
	/// Decrement the value at the pointer.
	/// Wraps around from 0 to 255.
	pub fn decr(&mut self) {
		self.data[self.idx] = self.data[self.idx].wrapping_sub(1);
	}
	
	
	/// Output (return) the value of the given cell as a char (ASCII-encoded u8).
	pub fn out(&self) -> u8 {
		self.data[self.idx]
	}
	
	/// Read in a given u8 and set the current cell to that value.
	pub fn inp(&mut self, inp: u8) {
		self.data[self.idx] = inp;
	}
	
	
	/// Helper method indicating whether the current cell is zero. Used for looping.
	pub fn is_zero(&self) -> bool {
		self.data[self.idx] == 0
	}
}


/// Fancily display the current cell and it's immediate neighbourhood.
impl Display for BFState {
	
	
	/// Format the current state nicely.
	fn fmt(&self, f: &mut Formatter) -> Result {
		let left_bound = match self.idx {  // Don't need to consider right bound as vector extends automatically
			idx if idx < 5 => - (idx as isize),
			_ => -5
		};
		let mut i = left_bound;
		
		write!(f, "\n{}\n\n", "State".bold().underline())?;
		write!(f, "Current position: {}\n\n", (self.idx as isize) - ((self.data.len() / 2) as isize))?;
		
		write!(f, "{:-<1$}\n", "", 62)?;
				
		if left_bound + self.idx as isize > 0 {
			write!(f, "...")?;
		}
		
		while i < 6 + (left_bound + 5) {
			let loc = (self.idx as isize + i) as usize;
			write!(f, "|")?;
			let output = match i {
				0 => self.data[loc].to_string().on_blue().bold(),
				_ => self.data[loc].to_string().on_truecolor(50, 50, 50),
			};
			write!(f, "{:4}", output)?;
			i += 1;
		}
		
		write!(f, "|...\n{:-<1$}\n", "", 62)?;
		
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use crate::state::BFState;

	#[test]
	fn test_init() {
		let state = BFState::new();
		assert_eq!(state.idx, 15_000);
		assert!(state.is_zero());
	}
	
	#[test]
	fn test_incr_decr() {
		let mut state = BFState::new();
		for _i in 0..50 {
			state.incr();
		}
		assert_eq!(state.out() as char, '2');  // converts to correct ascii
		for _i in 0..16 {
			state.incr();
		}
		assert_eq!(state.out() as char, 'B');  // shift up is correct
		for _j in 0..5 {
			state.decr();
		}
		assert_eq!(state.out() as char, '=');  // shift down is correct
		for _i in 0..256 {
			state.incr();
		}
		assert_eq!(state.out() as char, '=');  // wraps around for addition
		for _j in 0..256 {
			state.decr();
		}
		assert_eq!(state.out() as char, '=');  // wraps around for subtraction
	}
	
	#[test]
	fn test_up_down() {
		let mut state = BFState::new();
		for _i in 0..50 {
			state.incr();
		}
		state.up();
		assert!(state.is_zero());
		for _i in 0..66 {
			state.incr();
		}
		state.down();
		assert_eq!(state.out() as char, '2');
		state.up();
		assert_eq!(state.out() as char, 'B');
		state.up();
		assert!(state.is_zero());
	}
	
	#[test]
	fn test_inp() {
		let mut state = BFState::new();
		
		state.inp(',' as u8);
		for _i in 0..44 {
			state.decr();
		}
		assert!(state.is_zero());
	}
	
	#[test]
	fn test_extend() {
		let mut state = BFState::new();
		
		assert_eq!(state.data.len(), 30_000);
		for _ in 0..15_001 {
			state.up();
		}
		assert_eq!(state.data.len(), 35_000);
	}
}