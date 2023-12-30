pub mod cube_counter;
use cube_counter::*;
use std::error::Error;

fn main() {
    let lines = read_doc("input.txt").unwrap();

    //let mut total  = 0;
    //let mut gameNumber = 1;

    //println!("{:?}", lines);
    
    let mut line_edit: Vec<String>= Vec::new();
    for  line in lines{
        line_edit.push(line[5..].to_string());
    }
    
    println!("Total of the first part: {:?}", part1(line_edit.clone()).unwrap());

    
    println!("Total of the second part: {:?}", part2(line_edit.clone()).unwrap());
    

}

fn part1(line_edit: Vec<String>) -> Result<i32, Box<dyn Error>>{
    let mut total = 0;
    for line in line_edit{
        let result = cube_counter::line_test(&line).unwrap();
        if result.0 {
            total += result.1;
        }
    }
    Ok(total)
}

fn part2(line_edit: Vec<String>) -> Result<i32, Box<dyn Error>> {
    let mut total = 0;
    for line in line_edit {
        total += cube_counter::track_minimum_cubes(&line).unwrap();
    }
    Ok(total)
}



#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn test_1(){
        assert!(cube_counter::line_test("1: 6 blue, 8 red, 10 green; 12 blue, 10 red;").unwrap() == (true, 1));
        assert!(cube_counter::line_test("2: 14 blue, 12 red, 13 green;").unwrap() == (true, 2));
        assert!(cube_counter::line_test("2: 15 blue, 12 red, 13 green;").unwrap() == (false, 2));
        assert!(cube_counter::line_test("2: 14 blue, 13 red, 13 green;").unwrap() == (false, 2));
        assert!(cube_counter::line_test("2: 14 blue, 12 red, 14 green;").unwrap() == (false, 2));
        assert!(cube_counter::line_test("2: 14 blue, 12 red, 13 green; 15 green;").unwrap() == (false, 2));
    }
}