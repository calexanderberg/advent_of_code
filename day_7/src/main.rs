
fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let mut total = 0;
    let destroy_amount = 100000;

    let total_space = 70000000;
    let size_space = 30000000;
    let mut dir_to_delete = vec![];

    let mut matrix = vec![("/", 0)];
    for line in file.lines().filter(|l| !l.is_empty()) {
        if line == "$ cd /" || line == "$ ls" {
            continue;
        }

        if line.starts_with("$ cd ") {
            let dir = &line[5..];
            if dir == ".." {
                let (name, amount) = matrix.pop().unwrap();
                if amount <= destroy_amount {
                    total += amount;
                } 
                matrix.last_mut().unwrap().1 += amount;
                dir_to_delete.push((name, amount));
            } else {
                matrix.push((dir,0));
            }
            continue; 
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            matrix.last_mut().unwrap().1 += amount;
        }
    }

    while matrix.len() > 0 {
        let (name, amount) = matrix.pop().unwrap();
        dir_to_delete.push((name, amount));

        if matrix.len() > 0 {
            matrix.last_mut().unwrap().1 += amount;
        }
    }


    let free_space = total_space - dir_to_delete.last().unwrap().1;
    let required_space= size_space - free_space;

    let part2 = dir_to_delete
        .into_iter()
        .filter(move |(_, amount)| *amount > required_space)
        .map(|(_, amount)| {
            return amount;
        })
        .min();


    println!("{:?}", total);
    println!("{:?}", part2.unwrap());
}