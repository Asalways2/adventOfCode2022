
use std::fs;

/// elf rules
/// Opponent moves:
/// A Rock
/// B Paper
/// C Scissors
/// My moves:
/// X rock
/// Y Paper
/// Z scissors
/// 
/// Points per move
/// Rock 1
/// Paper 2
/// Scissors 3
/// 
/// winning move = 6
/// draw = 3
/// lose 0




fn play_the_game(puzzle_input: String, part2: bool) -> usize{

    let move_list: Vec<String> = puzzle_input.split("\n").map(|s| s.to_string()).collect();

    // keep counter for my points
    let mut points: usize = 0;
    for moves in move_list {
        // get the opponent move and my move
        let mut my_move = moves.split_whitespace().last().unwrap();
        let opponent_move = moves.split_whitespace().next().unwrap();
        
        if part2 {
            // now second collumn means your outcome
            // X => lose
            // Y => draw
            // Z => win
           match my_move {
            "X" => {
                match opponent_move {
                    "A" => {
                        my_move = "Z"
                    } // rock
                    "B" => {
                        my_move = "X"
                    } // paper
                    "C" => {
                        my_move = "Y"
                    } // scissors
                    &_ => {panic!("Unknown move")}
                }
            } // lets lose
            "Y" => {
                match opponent_move {
                    "A" => {
                        my_move = "X"
                    } // rock
                    "B" => {
                        my_move = "Y"
                    } // paper
                    "C" => {
                        my_move = "Z"
                    } // scissors
                    &_ => {panic!("Unknown move")}
                }
            } // draw
            "Z" => {
                match opponent_move {
                    "A" => {
                        my_move = "Y"
                    } // rock
                    "B" => {
                        my_move = "Z"
                    } // paper
                    "C" => {
                        my_move = "X"
                    } // scissors
                    &_ => {panic!("Unknown move")}
                }
            } // win
            &_ => {panic!("Unknown move")}

           }
        }

        println!("my move = {:?} opponent move = {:?}",my_move, opponent_move);

        // first get my base points based on the move i made
        match my_move {
            "X" => {points += 1} // rock        +   1
            "Y" => {points += 2} // paper       +   2
            "Z" => {points += 3} // scissors    +   3
            &_ => {panic!("Unknown move")}
        }

        match opponent_move {
            //rock
            "A" => {
                match my_move {
                    "X" => {points += 3} // rock draw
                    "Y" => {points += 6} // paper win
                    "Z" => {points += 0} // scissors lose
                    &_ => {panic!("Unknown move")}
                }
            }
            //paper
            "B" => {
                match my_move {
                    "X" => {points += 0} // lose
                    "Y" => {points += 3} // draw
                    "Z" => {points += 6} // win
                    &_ => {panic!("Unknown move")}
                }
            }
            //Scissors
            "C" => {
                match my_move {
                    "X" => {points += 6} // lose
                    "Y" => {points += 0} // draw
                    "Z" => {points += 3} // win
                    &_ => {panic!("Unknown move")}
                }
            }
            &_ => {panic!("Unkown move")}
        }
        // while solving this noticing that it follows xor behavior
        // [0   0] 0 draw
        // [1   0] 1 win/lose
        // [0   1] 1 win/lose
        // [1   1] 0 draw
        
    }
    
    points
}

fn main(){
    
}



#[cfg(test)]
mod tests {
    use super::*;
    

    #[test]
    fn test_1() {
        let puzzle_input_path = "puzzle_inputs/day2.txt";
        let puzzle_input = fs::read_to_string(puzzle_input_path).expect("something went wrong");
        
        assert_eq!(play_the_game(puzzle_input, false), 10941);
    }
    #[test]
    fn test_2(){
        let puzzle_input_path = "puzzle_inputs/day2.txt";
        let puzzle_input = fs::read_to_string(puzzle_input_path).expect("something went wrong");


        assert_eq!(play_the_game(puzzle_input, true), 13071);

    }
}