use md5;

//Main function
fn main() {

	let data = "bgvyzdsv";

	let mut i = 1;

	loop {
	
		let try_string = format!("{}{}",data, i);

		let result = md5::compute(try_string);

		if result[0] == 0 && result[1] == 0 && result[2] == 0 {
			println!("Found at {}", i);
			println!("{:x}", result);
			break;
		}

		i += 1;
	}	
}
