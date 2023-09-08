use std::{fs};

fn read_input_txt() -> String {
    let contents = fs::read_to_string("../input.txt").expect("Could not read the input.txt file");
    return contents;
}

fn main() {
    let input = read_input_txt();
    let elves = input.split("\n\n");
    
    let mut elf_cals_vec = Vec::<i64>::new();
    for elf in elves {
        let mut elf_cals: i64 = 0;
        // string is trimmed so we dont get an empty string
        // due to the newline at the end of file
        for food_item in elf.trim().split("\n") { 
            let food_cals: i64 = food_item.parse().unwrap();
            elf_cals += food_cals;
        }
        elf_cals_vec.push(elf_cals)
    }
    elf_cals_vec.sort();
    let vec_len = elf_cals_vec.len();
    let mut top_3_cals: i64 = 0;
    top_3_cals += elf_cals_vec[vec_len - 1];
    top_3_cals += elf_cals_vec[vec_len - 2];
    top_3_cals += elf_cals_vec[vec_len - 3];
    println!("{top_3_cals}");
}
