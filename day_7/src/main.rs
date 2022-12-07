fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    let destroy_amount = 100000;

    let mut matrix = vec![("/", 0)];
    for line in file.lines().filter(|l| !l.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (_, amount) = matrix.pop().unwrap();
                if amount <= destroy_amount {
                    total += amount;
                } 
                matrix.last_mut().unwrap().1 += amount;
            } else {
                matrix.push((dir,0));
            }
            continue;
        }

        let (amount, name) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            matrix.last_mut().unwrap().1 += amount;
        }
    }

    println!("{:?}", total);
}


/*
I realized way too late this can be done recursively. Need to rewrite this. 
*/

