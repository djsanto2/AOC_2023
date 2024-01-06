pub mod scratch_cards;
use scratch_cards::*;

fn main() {
    
    let lines = scratch_cards::read_doc("input.txt").unwrap();
    
    println!("{:?}", scratch_cards::part_one(&lines.clone()).unwrap());
    println!("{:?}", scratch_cards::part_two(&lines.clone()).unwrap());
    
}


#[cfg(test)]
mod tests_part1{

    use super::*;

    #[test]
    fn test_simple() {
        let lines = scratch_cards::read_doc("tests.txt").unwrap();
        assert!(scratch_cards::part_one(&lines.clone()).unwrap() == 13);
    }

    #[test]
    fn test_input() {
        let lines = scratch_cards::read_doc("input.txt").unwrap();
        for (num,line) in lines.lines().enumerate(){
            if num == 0 {
                assert!(scratch_cards::part_one(line).unwrap() == 512);
            }
            if num == 2 {
                assert!(scratch_cards::part_one(line).unwrap() == 16)
            }
        }
    }
}