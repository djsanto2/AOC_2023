use std::collections::VecDeque;
use std::fs;
use std::error::Error;




pub fn read_doc(filename: &str) -> Result<Vec<String>, Box<dyn Error>>{
    
    /*let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
    */
   
    Ok(fs::read_to_string(filename).unwrap().lines().into_iter().map(|x: &str| x.to_string()).collect())

}

pub fn line_test(line: &str) -> Result<(bool,i32), Box<dyn Error>>{
    
    let mut game_number = 0;
    //let bag = bag_of_cubes{red_cubes_ct: 12,green_cubes_ct: 13,blue_cubes_ct:14};

    let mut line_deque: VecDeque<char> = VecDeque::new(); 

    //GET GAME NUMBER FOR counter
    for c in line.chars(){
        if !c.is_whitespace(){
            line_deque.push_back(c);
        }
    }
    let mut current_digits = "".to_string();
    while *line_deque.front().unwrap() != ':'{
        current_digits.push(line_deque.pop_front().unwrap());
    }
    game_number = current_digits.parse::<i32>().unwrap();
    line_deque.pop_front(); //pops the :


    //check if the game was valid or trickery
    let mut word = "".to_string();
    let mut current_count  = 0;
    let mut toggle = true;
    while !line_deque.is_empty(){
        let current_item = line_deque.pop_front().unwrap();
        if current_item.is_numeric(){
            word.push(current_item);
            toggle = true;
        }
        else if current_item.is_alphabetic() && !word.is_empty() {
            if toggle {
                current_count = word.parse::<i32>().unwrap();
                word.clear();
                toggle = false;
            }
            word.push(current_item);
        }
        else if current_item == ',' || current_item == ';' {
            if word == "red"{
                if current_count > 12 {
                    return Ok((false,game_number))
                }
                word.clear();
                current_count = 0;
            }
            else if word == "green" {
                if current_count > 13 {
                    return Ok((false,game_number))
                }
                word.clear();
                current_count = 0;
            }
            else if word == "blue" {
                if current_count > 14 {
                    return Ok((false,game_number))
                }
                word.clear();
                current_count = 0;
            }
        }
        if !word.is_empty(){
            if word == "red"{
                if current_count > 12 {
                    return Ok((false,game_number))
                }
                word.clear();
                current_count = 0;
            }
            else if word == "green" {
                if current_count > 13 {
                    return Ok((false,game_number))
                }
                word.clear();
                current_count = 0;
            }
            else if word == "blue" {
                if current_count > 14 {
                    return Ok((false,game_number))
                }
                word.clear();
                current_count = 0;
            }
        }
    }

    Ok((true,game_number))
}