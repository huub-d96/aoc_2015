use regex::Regex;
use std::fs;

fn main() {

	let mut nice1 = 0;
	let mut nice2 = 0;
	let data = fs::read_to_string("src/data.txt").expect("Failed to read");


	for line in data.lines() {

		//Part 1
		//Check for vowels
		let vowels = Regex::new(r"[aeiou]").unwrap();
		let vowel_matches: Vec<_> = vowels.find_iter(line).map(|k| k.as_str()).collect();

		//Check for double letters
		let doubles = Regex::new(r"aa|bb|cc|dd|ee|ff|gg|hh|ii|jj|kk|ll|mm|nn|oo|pp|qq|rr|ss|tt|uu|vv|ww|xx|yy|zz").unwrap();
		let double_matches: Vec<_> = doubles.find_iter(line).map(|k| k.as_str()).collect();

		//Check for forbidden letters
		let forbidden = Regex::new(r"ab|cd|pq|xy").unwrap();
		let forbidden_matches: Vec<_> = forbidden.find_iter(line).map(|k| k.as_str()).collect();

		if vowel_matches.len() >= 3 && !double_matches.is_empty() && forbidden_matches.is_empty() {
			nice1 += 1;
		}

		//Part 2
		let letters = line.chars().collect::<Vec<char>>();
		
		//Check for pairs
		let mut nice_pair = false;
		for i in 0..letters.len()-2 {
			let p1 = letters[i];
			let p2 = letters[i+1];

			for j in i+2..letters.len()-1 {
				if p1 == letters[j] && p2 == letters[j+1] {
					nice_pair = true;
				}
			}		
		}

		//Check for repeats
		let mut nice_repeat = false;
		for i in 0..letters.len()-2 {
			let p1 = letters[i];
			let p2 = letters[i+2];

			if p1 == p2 {
				nice_repeat = true;
			}		
		}

		if nice_pair && nice_repeat {
			nice2 += 1;
		}
		
		
	}

println!("Part 1: {}", nice1);
println!("Part 2: {}", nice2);
    
}
