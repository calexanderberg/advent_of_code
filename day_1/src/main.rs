use std::fs::File;
use std::vec;
use std::io::{BufRead, BufReader};

fn main() {
    let elf_list: Vec<i32> = elf_list_create("calories.txt");
    // For tomorrow, look into why you can't reuse vectors.
    let elf_list1 = elf_list.clone();
    let elf_list2 = elf_list.clone();

    println!("{:?}", elf_list);

    let packed_elf: i32 = most_packed_elves(elf_list1 , 1);

    println!("\nThe most packed elf is packing {} calories", packed_elf);

    let packed_elves: i32 = most_packed_elves(elf_list2, 3);

    println!("\nThe three most packed elves are packing {} calories", packed_elves);
}

fn elf_list_create(filename: &str) -> Vec<i32> {
    // Basic stuff for reading the files
    let filename = filename;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut elf_list= vec![0];

    // For creating our elf_list
    let mut i = 0;
    for (_index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); 
        if line == "" {
            i = i + 1;
            elf_list.push(0)
        } else {
            let int = line.parse::<i32>().unwrap();
            elf_list[i] = elf_list[i] + int;
        }
    }

    elf_list.sort();  // Sorts the list with the last value being largest.

    return elf_list;
}

// This function assumes that the vector is sorted
fn most_packed_elves(vector: Vec<i32>, num: usize) -> i32 {
    let mut total_calories = 0;
    for a in 1..num+1 {total_calories = total_calories + vector[vector.len() - a]};
    return total_calories;
}
