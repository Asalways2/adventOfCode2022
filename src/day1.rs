
use std::fs;




#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct ElfBackpack{
    snacks: Vec<String>,
    snack_sum: usize,
}

/**
docs


```
assert_eq!(false , false); 
```
*/
impl ElfBackpack {

    fn new(snacks: &[String]) -> ElfBackpack{
        let mut sum: usize = 0;
        
        for snack in snacks {
            sum += snack.parse::<usize>().unwrap_or(0);
        }


        ElfBackpack{snacks: snacks.to_owned(), snack_sum: sum}

        
    }

}




fn main() -> usize{
    let mut elf_expedition : Vec<ElfBackpack> = Vec::new();
    let path_to_snack_list = "puzzle_inputs/day1-1.txt";

    let raw_snack_list = fs::read_to_string(path_to_snack_list).expect("something went wrong");
    

    let split_snacks: Vec<String> = raw_snack_list.split("\n").map(|s| s.to_string()).collect();
    

    let mut snack_buf :Vec<String> = Vec::new();

    for snack in split_snacks {

        if snack.is_empty() {
            // no snack so must be another backpack
            // clear the snack buffer and put it in an ElfBackpack
            let backpack: ElfBackpack = ElfBackpack::new(&snack_buf);
            snack_buf.clear();

            
            
            elf_expedition.push(backpack);

        }else{
            snack_buf.push(snack);
        }
        
    }

    // let mut highest : usize = 0;
    // for backpack in elf_expedition{

    //     if backpack.snack_sum > highest {
    //         highest = backpack.snack_sum;
    //     }
    // }
    //elf_expedition.sort();

    
    elf_expedition.sort_by(|a, b| b.snack_sum.cmp(&a.snack_sum));
    print!("{:?}", elf_expedition);
    

    


    let mut sum : usize = 0;
    for n in 0..3{
        println!("{:?}", elf_expedition[n]);
        sum += elf_expedition[n].snack_sum;
    }
    println!("calorie total of 3 bigest backpacks = {}",sum);

    sum


 

}


// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn day01() {
//         assert_eq!(, 203420);
//     }
// }