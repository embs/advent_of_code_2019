use std::cmp;

fn walk(from: [i32;2], direction: char, steps: i32) -> [i32;2] {
    if direction == 'U' {
        [from[0], from[1] + steps]
    } else if direction == 'R' {
        [from[0] + steps, from[1]]
    } else if direction == 'D' {
        [from[0], from[1] - steps]
    } else if direction == 'L' {
        [from[0] - steps, from[1]]
    } else {
        panic!("Unknown direction: {}", direction);
    }
}

fn main() {
    // Example. Answer: 6
    let wire_1_path = "R8,U5,L5,D3";
    let wire_2_path = "U7,R6,D4,L4";

    // Sample 1. Answer: 159
    //let wire_1_path = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    //let wire_2_path = "U62,R66,U55,R34,D71,R55,D58,R83";

    // Sample 2. Answer: 135
    //let wire_1_path = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    //let wire_2_path = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    let mut wire_1_position = [0,0];
    let mut wire_2_position = [0,0];
    let mut closest_intersection_distance = 0;

    let mut wire_1_walks: Vec<String> = wire_1_path.split(",").map(|s| s.to_string()).collect();
    let mut wire_2_walks: Vec<String> = wire_2_path.split(",").map(|s| s.to_string()).collect();

    let smaller_walk = cmp::min(wire_1_walks.len(), wire_2_walks.len());

    for i in 0..smaller_walk {
        let mut wire_1_walk = &mut wire_1_walks[i];
        let mut wire_2_walk = &mut wire_2_walks[i];

        wire_1_position = walk(wire_1_position, wire_1_walk.remove(0), wire_1_walk.parse().unwrap());
        wire_2_position = walk(wire_2_position, wire_2_walk.remove(0), wire_2_walk.parse().unwrap());
        
        for x in &wire_1_position {
            print!("{},", x);
        }
        print!(" ");
        for x in &wire_2_position {
            print!("{},", x);
        }
        println!();

        if wire_1_position[0] == wire_2_position[1] {
            let distance = wire_2_position[0].abs() + wire_1_position[1].abs();

            if closest_intersection_distance == 0 || distance < closest_intersection_distance {
                closest_intersection_distance = distance;
            }
        }

        if wire_1_position[1] == wire_2_position[0] {
            let distance = wire_2_position[1].abs() + wire_1_position[0].abs();

            if closest_intersection_distance == 0 || distance < closest_intersection_distance {
                closest_intersection_distance = distance;
            }
        }
    }

    println!("{}", closest_intersection_distance);
}
