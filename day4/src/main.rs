pub mod scratch_cards;
use scratch_cards::*;

fn main() {
    
    let lines = scratch_cards::read_doc("input.txt").unwrap();

    println!("{:?}", scratch_cards::part_one(&lines.clone()).unwrap());
    
}
