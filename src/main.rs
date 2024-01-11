mod state;
mod parser;

fn check_valid_inp(buf: String) -> bool {
	buf.chars().fold(0, |acc, x| match x {'[' => acc + 1, ']' => acc - 1, _ => acc}) == 0
}

fn main() {
	let mut p = parser::Parser::new();
	let mut buf = String::new();
	let mut inp_buf = String::new();
	
	while buf.trim() != "quit" {
		if buf.contains(",") {
			inp_buf.clear();
			println!("Please enter program input:");
			std::io::stdin().read_line(&mut inp_buf).expect("oops");
		}
		
		p.parse(buf.trim().to_string(), inp_buf.trim().to_string()).expect("Failed to parse");
		println!("{p}");
		
		buf.clear();
		println!("Next command:");
		std::io::stdin().read_line(&mut buf).expect("oops");
		while !check_valid_inp(buf.clone()) { // Wait for closing bracket
			std::io::stdin().read_line(&mut buf).expect("oops");
		}
	}
}
