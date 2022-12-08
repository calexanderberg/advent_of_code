fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();

    let mut sum = 0;
    let mut forrest: Vec<Vec<u32>> = vec![];
		let mut best_score = 0;
		let mut temp_score = 1;

    for line in file.lines().filter(|l| !l.is_empty()) { 
        forrest.push(line.chars()
                   .map(|d| d.to_digit(10).unwrap())
                   .collect());
    }

    for i in 0..forrest.len() {
        if i == 0 || i == forrest.len()-1 {
           continue;
        } else {
            for a in 1..forrest[i].len()-1 {
                let mut result = 0;
                for n in 1..5 {
                    result = check_direction(n, forrest[i][a],(*forrest).to_vec(), [i, a]);

                    if result == 1 {
                        break;
                    }
                }
                sum += result;
            }

						for a in 1..forrest[i].len()-1 {
							temp_score = 1;
							for n in 1..5 {
									let temp = check_score(n, forrest[i][a],(*forrest).to_vec(), [i, a]); 
									temp_score *= temp;
							}
							if temp_score > best_score {
								best_score = temp_score;
							}						
						}
					}
				}
	// Add first and last row of trees
	sum += forrest[0].len() * 2;
	// Add sides
	sum += forrest.len()*2;
	// Remove duplicates
	sum -= 4;	
	println!("Sum: {}", sum);
  println!("Best score: {}", best_score);
}


/* 
Tried to merge functional recursion like code with rust. It was fun, I wonder if I should od something similar to the main function.
*/
fn check_direction(direction: u32, from_val: u32, forrest: Vec<Vec<u32>>, [val1, val2]: [usize; 2]) -> usize {
	let mut result = 0;
    if val1 == 0 || val2 == 0 || val1 == forrest.len()-1 || val2 == forrest[val1].len()-1 {
        result = 1;
    } else {
        match direction {
            // Right
            1 => {
                if from_val > forrest[val1][val2+1]{
                    result = check_direction(1, from_val, forrest, [val1, val2+1]);
                }
            },
            //Left
            2 => {
                if from_val > forrest[val1][val2-1] {
                    result = check_direction(2, from_val, forrest, [val1, val2-1]);
                } 
            },
            //Top
            3 => {
                if from_val > forrest[val1-1][val2] {
                    result = check_direction(3, from_val, forrest, [val1-1, val2]);
                } 
            },
            //Bottom
            4 => {
                if from_val > forrest[val1+1][val2] {
                    result = check_direction(4, from_val, forrest, [val1+1, val2]);
                } 
            },
            _ => panic!(),
        }
    }
    return result;
}

fn check_score(direction: u32, from_val: u32, forrest: Vec<Vec<u32>>, [val1, val2]: [usize; 2]) -> usize {
	let mut result = 1;
    if val1 == 0 || val2 == 0 || val1 == forrest.len()-1 || val2 == forrest[val1].len()-1 {
        result = 0;
    } else {
        match direction {
            // Right
            1 => {
                if from_val > forrest[val1][val2+1]{
                    result += check_score(1, from_val, forrest, [val1, val2+1]);
                }
            },
            //Left
            2 => {
                if from_val > forrest[val1][val2-1] {
                    result += check_score(2, from_val, forrest, [val1, val2-1]);
                } 
            },
            //Top
            3 => {
                if from_val > forrest[val1-1][val2] {
                    result += check_score(3, from_val, forrest, [val1-1, val2]);
                } 
            },
            //Bottom
            4 => {
                if from_val > forrest[val1+1][val2] {
                    result += check_score(4, from_val, forrest, [val1+1, val2]);
                } 
            },
            _ => panic!(),
        }
    }
    return result;
}
