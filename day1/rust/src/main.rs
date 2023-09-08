use std::fs;

fn read_input_txt() -> String {
    let contents = fs::read_to_string("../input.txt").expect("Could not read the input.txt file");
    return contents;
}

fn main() {
    let input = read_input_txt();
    let elves = input.split("\n\n");
    
    let mut max_cals = 0;
    for elf in elves {
        let mut elf_cals: i64 = 0;
        // string is trimmed so we dont get an empty string
        // due to the newline at the end of file
        for food_item in elf.trim().split("\n") { 
            let food_cals: i64 = food_item.parse().unwrap();
            elf_cals += food_cals;
        }
        if elf_cals > max_cals {
            max_cals = elf_cals;
        }
    }
    println!("{max_cals}");
}
