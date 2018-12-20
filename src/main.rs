use itertools::Itertools;
use structopt::StructOpt;

mod display;
use crate::display::*;

#[derive(StructOpt)]
struct Options {
	/// Decrypt instead of encrypt.
	#[structopt(short, long)]
	decrypt: bool,

	/// The text to encrypt or decrypt.
	text: String,

	/// The key.
	#[structopt(short, long)]
	key: String,

	/// Show the encryption or decryption steps in detail.
	#[structopt(short, long)]
	verbose: bool,
}

fn letter_filter<'a>(input: &'a str) -> impl Iterator<Item = char> + 'a {
	input
		.as_bytes()
		.iter()
		.map(u8::to_ascii_uppercase)
		.filter(|&x| x >= b'A' && x <= b'Z')
		.map(|x| if x == b'J' { 'I' } else { x as char })
}

pub struct Table {
	pub cells: [u8; 25],
}

impl Table {
	fn new(key: &str) -> Table {
		let letters = letter_filter(key)
			.map(|x| x as u8)
			.chain(b'A'..=b'I')
			.chain(b'K'..=b'Z');
		let mut i = 0;
		let mut cells = [0u8; 25];
		for l in letters {
			if !cells[0..i].iter().any(|&x| x == l) {
				cells[i] = l;
				i += 1;
			}
		}
		assert!(i == 25);
		Table { cells }
	}

	fn char(&self, coord: (i32, i32)) -> char {
		let index = coord.0 * 5 + coord.1;
		self.cells[index as usize] as char
	}

	fn coord(&self, c: char) -> (i32, i32) {
		let index = self.cells.iter().position(|&x| x as char == c).unwrap() as i32;
		(index / 5, index % 5)
	}
}

fn main() {
	let opt = Options::from_args();

	let table = Table::new(&opt.key);

	if opt.verbose {
		println!("Key table:");
		display_table(&table);
	}

	let input: Vec<(char, (i32, i32))> = letter_filter(&opt.text)
		.map(|l| (l, table.coord(l)))
		.collect();

	if opt.verbose {
		println!("\n{}:", if opt.decrypt { "Decrypting" } else { "Encrypting" });
		display_letter_coords(&input, "LRC");
	}

	let rows = input.iter().map(|&(_, (r, _))| r);
	let cols = input.iter().map(|&(_, (_, c))| c);

	let output: Vec<(char, (i32, i32))> = if opt.decrypt {
		let indexes: Vec<i32> = rows.interleave(cols).collect();
		let (rows, cols) = indexes.split_at(indexes.len() / 2);
		rows.iter()
			.zip(cols)
			.map(|(&r, &c)| (table.char((r, c)), (r, c)))
			.collect()
	} else {
		rows.chain(cols)
			.tuples()
			.map(|coord| (table.char(coord), coord))
			.collect()
	};

	if opt.verbose {
		println!("\nRe-ordered:");
		display_letter_coords(&output, "RCL");
		print!("\n{}: ", if opt.decrypt { "Decrypted" } else { "Encrypted" });
	}

	for &(l, _) in &output {
		print!("{}", l);
	}
	println!();
}
