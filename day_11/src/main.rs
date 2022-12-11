use std::str::FromStr;

fn main() {
   let _test_monkeys: Vec<Vec<&str>> = vec![
    vec!["0", "79 98", "old * 19", "23", "2", "3"],
    vec!["1", "54 65 75 74", "old + 6", "19", "2", "0"],
    vec!["2", "79 60 97", "old * old", "13", "1", "3"],
    vec!["3", "74", "old + 3", "17", "0", "1"],
   ];

   let _input_monkeys: Vec<Vec<&str>> = vec![
    vec!["0", "66 59 64 51", "old * 3", "2", "1", "4"],
    vec!["1", "67 61", "old * 19", "7", "3", "5"],
    vec!["2", "86 93 80 70 71 81 56", "old + 2", "11", "4", "0"],
    vec!["3", "94", "old * old", "19", "7", "6"],
    vec!["4", "71 92 64", "old + 3", "3", "5", "1"],
    vec!["5", "58 81 92 75 56", "old + 6", "5", "3", "6"],
    vec!["6", "82 98 77 94 86 81", "old + 7", "17", "7", "2"],
    vec!["7", "54 95 70 93 88 93 63 50", "old + 4", "13", "2", "0"],
   ];

   monkey_round(_test_monkeys, 20);
}

fn monkey_round(mut monkeys: Vec<Vec<&str>>, rounds: u32) -> u32{
    let mut passes = 0;
    let mut inspect: Vec<u32> = vec![0,0,0,0,0,0,0,0,0];
   
    for _a in 0..rounds+1 {
        for n in 0..monkeys.len() {
            let items: Vec<&str> = monkeys[n][1].clone().split(" ").collect();
            let instruction: Vec<&str> = monkeys[n][2].clone().split(" ").collect();
            for x in 0..items.len() {
                let item = items[x];
                let new_val = operate_item(instruction.clone(), item);
                if test_item(new_val, monkeys[n][3]) {
                    let throw_to_monkey: usize = FromStr::from_str(monkeys[n][4].clone()).unwrap();
                    monkeys[throw_to_monkey] = throw_item(monkeys[throw_to_monkey].clone(), item);
                    inspect[n] += 1;
                    monkeys[n] = remove_item(monkeys[n].clone(), item);
                } else {
                    let throw_to_monkey: usize = FromStr::from_str(monkeys[n][5].clone()).unwrap();
                    monkeys[throw_to_monkey] = throw_item(monkeys[throw_to_monkey].clone(), item);
                    inspect[n] += 1;
                    monkeys[n] = remove_item(monkeys[n].clone(), item);
                }
            }
        }
    }

    for a in 0..inspect.len() {
        passes *= inspect[a];
    }
    println!("Passes: {}", passes);
    return passes
}


fn operate_item(instruction: Vec<&str>, item: &str) -> u32 {
    let value1: u32 = FromStr::from_str(item).unwrap();
    let value2: u32;
    if instruction[2] == "old" {
        value2 = FromStr::from_str(item).unwrap();
    } else {
        value2 = FromStr::from_str(instruction[2]).unwrap();
    }

    if instruction[1] == "+" {
        return value1 + value2;
    } else {
        return value1 + value2;
    }
}

fn test_item(item: u32, test_value: &str) -> bool {
    let val2: u32 = FromStr::from_str(test_value).unwrap();
    return item % val2 == 0
}
 
fn throw_item<'a>(monkey: Vec<&'a str>, item: &str) ->  Vec<&'a str> {
    let mut new_monkey = monkey.clone();
    let mut new_val = new_monkey[1].to_owned();
    new_val.push_str(" ");
    new_val.push_str(item);
    new_monkey[1] = new_val.as_str();
    print!("item: {}, thrown to {:?}", item, new_monkey[0]);
    return new_monkey;
}
fn remove_item<'a>(mut monkey: Vec<&'a str>, item: &'a str) -> Vec<&'a str> {
    let mut items: Vec<&str> = monkey[1].clone().split(" ").collect();
    let index = items.iter().position(|x| x == &item).unwrap();
    items.remove(index);
    monkey[1] = &items.join(" ").to_string();
    return monkey;
}
