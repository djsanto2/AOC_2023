use std::{fs, error::Error, thread::current};




pub fn read_doc(filename: &str) -> Result<String, Box<dyn Error>>{
    
    /*let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
    */
    Ok(fs::read_to_string(filename).unwrap())

}

pub fn part_one(input : &str) -> Result<i32, Box<dyn Error>> {
    let mut current_winning_numbers: Vec<i32> = vec![];
    let mut current_play_numbers : Vec<i32> = vec![];
    let mut current_number = "".to_string();

    let mut scores: Vec<i32> = vec![];
    for (_card_number, line) in input.lines().enumerate() {
        let mut begin = false;
        let mut got_playing = false;
        for c in line.chars(){
            if begin {
                if c.is_numeric() {
                    current_number.push(c);
                }
                else if c.is_whitespace() {
                    if !got_playing && !current_number.is_empty() {
                        current_winning_numbers.push(current_number.parse::<i32>().unwrap());
                        current_number.clear();
                    }
                    else if got_playing && !current_number.is_empty(){
                        current_play_numbers.push(current_number.parse::<i32>().unwrap());
                        current_number.clear();
                    }
                }
                else if c == '|' {
                    got_playing = true;
                }
            }
            else if c == ':' {
                begin = true;
            }
        }
        //todo: Sum count of winners in play numbers
        scores.push(current_play_numbers.iter().filter(|x| current_winning_numbers.contains(x)).count() as i32);
    }
    Ok(scores.iter().sum())
}