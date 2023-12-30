use std::fs;
use std::error::Error;

struct bag_of_cubes{
    red_cubes_ct: i32,
    green_cubes_ct: i32,
    blue_cubes_ct: i32,
}


pub fn read_doc(filename: &str) -> Result<Vec<String>, Box<dyn Error>>{
    
    /*let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
    */
   
    Ok(fs::read_to_string(filename).unwrap().lines().into_iter().map(|x: &str| x.to_string()).collect())

}

pub fn line_test(line: &str) -> Result<(bool,i32), Box<dyn Error>>{
    Ok((true,6))
}