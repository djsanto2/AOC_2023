pub mod cube_counter;
use cube_counter::*;


fn main() {
    let lines = read_doc("input_shortened.txt").unwrap();

    //let mut total  = 0;
    //let mut gameNumber = 1;

    //println!("{:?}", lines);
    let mut line_edit: Vec<String>= Vec::new();
    for  line in lines{
        line_edit.push(line[5..].to_string());
    }
    for line in line_edit{
        let result = cube_counter::line_test(line);
    }
    //print!("{:?}", lines);
}
