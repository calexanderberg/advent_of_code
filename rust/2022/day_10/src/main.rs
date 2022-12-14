fn main() {
    let file = std::fs::read_to_string("input.txt").unwrap();
    let program: Vec<&str> = file.lines().collect();

    let cycle = build_cycle(vec![], program, 0);
    let signal = execute(cycle, 1, 0, 1, [1,2,3], 1);

    println!("\n\nValue of signal {}", signal);
}

fn build_cycle(mut cycle: Vec<i32>, program: Vec<&str>, index: usize) -> Vec<i32>{
    if program[index].starts_with("noop") {
        cycle.push(0);
    } else {
        let split: Vec<&str> = program[index].clone().split(" ").collect();
        let value: i32 = split[1].parse::<i32>().unwrap();
        cycle.push(0);
        cycle.push(value);
    }

    if index == program.len()-1 {
        return cycle
    } else {
        return build_cycle(cycle, program, index+1);
    }
}

fn execute(mut cycle: Vec<i32>, mut register: i32, mut signal: i32, mut counter: i32, mut sprite: [i32; 3], mut crt: i32) -> i32 {
    if cycle.len() == 0 {
        return signal;
    } else {
        print!("{}", draw_crt(crt, sprite));

        if counter == 20 || counter == 60 || counter == 100 || counter == 140 || counter == 180 || counter == 220 {
            signal += register * counter;
            }    
        
        if cycle[0] != 0{
            register = addx(register, cycle[0]);
        }

        sprite = update_sprite( register);

        cycle.remove(0);

        if counter % 40 == 0 {
            print!("\n");
            crt = 0;
        }
        counter += 1;
        crt += 1;
        return execute(cycle, register, signal, counter, sprite , crt);
    }

}


fn addx(register: i32, value: i32) -> i32 {
    let new_value: i32 = register + value;
    return new_value;
}

fn update_sprite(next_val: i32) -> [i32; 3] {
    let sprite = [next_val, next_val + 1 , next_val + 2];
    return sprite;
}

fn draw_crt(crt_post: i32, sprite: [i32; 3]) -> char {
    if sprite.contains(&crt_post) {
        return '🎁'
    } else {
        return '🎄'
    }
}