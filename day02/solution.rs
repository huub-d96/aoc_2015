use std::fs;

//Main function
fn main() {

	let data = "
	2x3x4
	1x1x10";

	let data = fs::read_to_string("data.txt").expect("Reading");

	let mut total_paper: u32 = 0;
	let mut total_ribbon: u32 = 0;
	
    for line in data.lines(){
    	if line != "" {

			let current_line = line.replace(" ","").replace("\t", "");
 			let sides = current_line.split('x').collect::<Vec<_>>();

 			let l:u32 = sides[0].parse().unwrap();
 			let w:u32 = sides[1].parse().unwrap();
 			let h:u32 = sides[2].parse().unwrap();	

			let mut sides = vec![l*w, w*h, h*l];
			let mut dims = vec![l, w, h];
			sides.sort();
			dims.sort();
			
			let paper = 2*sides.iter().sum::<u32>() + sides[0];
			let ribbon = 2*dims[0]+2*dims[1] + l*w*h;

			total_paper += paper;
			total_ribbon += ribbon;
    	}
    	
    }

    println!("Total paper: {}", total_paper);
    println!("Total ribbon: {}", total_ribbon);
}
