use crate::Table;

pub fn display_table(table: &Table) {
	println!("  │ 1 2 3 4 5");
	println!("──┼──────────");
	for row in 0..5 {
		print!("{} │", row + 1);
		for column in 0..5 {
			print!(" {}", table.cells[row * 5 + column] as char);
		}
		println!();
	}
}

pub fn display_letter_coords(letter_coords: &[(char, (i32, i32))], order: &str) {
	for o in order.chars() {
		match o {
			'L' => {
				print!("Letter │");
				for &(l, _) in letter_coords {
					print!(" {}", l);
				}
			}
			'R' => {
				print!("Row    │");
				for &(_, (r, _)) in letter_coords {
					print!(" {}", r + 1);
				}
			}
			'C' => {
				print!("Column │");
				for &(_, (_, c)) in letter_coords {
					print!(" {}", c + 1);
				}
			}
			_ => panic!(),
		}
		println!();
	}
}
