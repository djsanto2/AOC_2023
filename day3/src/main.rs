pub mod part_vector;
use part_vector::*;




fn main() {
    
    let lines = read_doc("input.txt").unwrap();

    println!("{:?}",part_one(lines).unwrap());
}