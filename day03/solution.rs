use std::fs;
use std::collections::HashMap;

//Main function
fn main() {

	//let data = "^v^v^v^v^v";
	let data = fs::read_to_string("data.txt").expect("Reading").replace("\n", "");
	
	let mut house_map = HashMap::from([((0,0), 1)]);
	let mut robot_map = HashMap::from([((0,0), 1)]);

	let dirs = HashMap::from([
			('>', (1,0)),
			('<', (-1,0)),
			('^', (0,1)),
			('v', (0,-1))
		]);

	let mut loc = (0,0);
	let mut santa_loc = (0,0);
	let mut robot_loc = (0,0);
	
    for (i,c) in data.chars().enumerate() {
    	let dir = dirs.get(&c).unwrap();

		//Part 1
    	loc = (loc.0+dir.0, loc.1+dir.1);

    	if house_map.contains_key(&loc) {
    		house_map.insert(loc, house_map[&loc] + 1 );
    	} else {
    		house_map.insert(loc, 1);
    	}

    	if i % 2 == 0 {
    		santa_loc = (santa_loc.0+dir.0, santa_loc.1+dir.1);
    		robot_map.insert(santa_loc, if robot_map.contains_key(&santa_loc) {robot_map[&santa_loc] + 1} else {1} );
    	} else {
    		robot_loc = (robot_loc.0+dir.0, robot_loc.1+dir.1);
    		robot_map.insert(robot_loc, if robot_map.contains_key(&robot_loc) {robot_map[&robot_loc] + 1} else {1} );    		
    	}

    	

		//println!("{:?} {:?}", dir, loc);

    }

    println!("Part 1 - Houses visited: {}", house_map.len());
    println!("Part 2 - Houses visited: {}", robot_map.len());
}
