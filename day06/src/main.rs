use std::fs;

fn main() {
    
	let data = fs::read_to_string("src/data.txt").expect("Failed to read");

    let mut lights:[bool; 1000*1000] = [false; 1000*1000];
    let mut bright_lights: [u8;1000*1000] = [0; 1000*1000];

    for line in data.lines() {

		let splits: Vec<_> = line.trim().split(' ').collect();

		if splits[0] == "turn"  {

			let start: Vec<_> = splits[2].split(',').collect();
			let end: Vec<_> = splits[4].split(',').collect();
			let state: bool = splits[1] == "on"; 

			for x in start[0].parse::<u32>().unwrap() .. end[0].parse::<u32>().unwrap() + 1 {
				for y in start[1].parse::<u32>().unwrap() .. end[1].parse::<u32>().unwrap() + 1 {
					lights[(1000*y + x) as usize] = state;

					if state {
						bright_lights[(1000*y + x) as usize] += 1;
					} else if bright_lights[(1000*y + x) as usize] > 0 {
						bright_lights[(1000*y + x) as usize] -= 1;
					}
				}
			}    	
    	}
    	else {
    		let start: Vec<_> = splits[1].split(',').collect();
			let end: Vec<_> = splits[3].split(',').collect();

			for x in start[0].parse::<u32>().unwrap() .. end[0].parse::<u32>().unwrap() + 1 {
				for y in start[1].parse::<u32>().unwrap() .. end[1].parse::<u32>().unwrap() + 1 {
					lights[(1000*y + x) as usize] = !lights[(1000*y + x) as usize];
					bright_lights[(1000*y + x) as usize] += 2;
				}
			}
    	}
    }

	let lights_on = lights.into_iter().filter(|b| *b).count();
	let mut brightness: u32 = 0;
	for i in 0 .. bright_lights.len() {
		brightness += bright_lights[i] as u32;
	}

	println!("Part 1: There are {} lights on", lights_on);
	println!("Part 2: Brightness is {}", brightness);
    
}
