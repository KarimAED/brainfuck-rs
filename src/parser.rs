use anyhow::{Context, Result, anyhow};
use crate::state;
use std::fmt;

pub struct Parser {
	current_pos: usize,
	loop_stack: Vec<usize>,
	state: state::BFState,
	skip_flag: isize
}

impl Parser {
	pub fn new() -> Parser {
		Parser {
			current_pos: 0,
			loop_stack: Vec::new(),
			state: state::BFState::new(),
			skip_flag: -1
		}
	}
	
	fn reset(&mut self) {
		self.current_pos = 0;
		self.loop_stack.clear();
		self.skip_flag = -1;
	}
	
	fn parse_next(&mut self, code: &String, input: &mut impl std::iter::Iterator<Item = char>) ->  Result<()>{
		let command = code[self.current_pos..].chars().next();
		match command {
			Some('[') => self.start_of_loop(),
			Some(']') => self.end_of_loop()?,
			_ => {}
		}
		if self.skip_flag != -1 {
			return Ok(())
		}
		match command {
			Some('+') => self.state.incr(),
			Some('-') => self.state.decr(),
			Some('>') => self.state.up(),
			Some('<') => self.state.down(),
			Some('.') => print!("{}", self.state.out() as char),
			Some(',') => {
				match input.next() {
					Some(i) => self.state.inp(i as u8),
					_ => {}
				}
			},
			_ => {}
		}
		
		Ok(())
	}
	
	pub fn parse(&mut self, code: String, input: String) ->  Result<()>{
		let mut input_iter = input.chars();
		self.reset();
		while self.current_pos < code.len() {
			self.parse_next(&code, &mut input_iter)?;
			self.current_pos += 1;
		}
		
		if self.loop_stack.len() != 0 {
			Err(anyhow!("Too few ] encountered in code."))
		} else {
			Ok(())
		}
	}
	
	fn start_of_loop(&mut self) {
		self.loop_stack.push(self.current_pos);
		if self.state.is_zero() && self.skip_flag == -1 {
			self.skip_flag = self.current_pos as isize;
		}
	}
	
	fn end_of_loop(&mut self) -> Result<()>{
		if self.skip_flag  != -1 {
			let last = self.loop_stack.pop().context("Invalid [ and ] placement.")?;
			if last == self.skip_flag as usize {
				self.skip_flag = -1;
			}
			return Ok(());
		}
		if self.state.is_zero() {
			self.loop_stack.pop();
		} else {
			self.current_pos = *self.loop_stack.last().context("Could not find a starting [ to jump to.")?;
		}
		
		Ok(())
	}
}

impl fmt::Display for Parser {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		write!(f, "{}", self.state)?;
		
		Ok(())
	}
}

