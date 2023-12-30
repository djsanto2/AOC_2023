use std::collections::VecDeque;
use std::fs;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
struct MyError(String);

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "Error Here: {}", self.0)
    }
}


struct BagOfCubes{
    red_ct: i32,
    green_ct: i32,
    blue_ct: i32,
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
    let game_number = current_digits.parse::<i32>().unwrap();
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


pub fn track_minimum_cubes(line: &str) -> Result<i32,Box<dyn Error>>{
    
    let line_deque: VecDeque<char> = str_to_deque(line).unwrap();

    let bag_cube = count_max_cubes(line_deque).unwrap();

    Ok(bag_cube.red_ct*bag_cube.blue_ct*bag_cube.green_ct)
}

fn str_to_deque(line: &str) -> Result<VecDeque<char>, Box<dyn Error>> {
    let mut result_vec: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        result_vec.push_back(c);
    }
    Ok(result_vec)
}

impl Error for MyError{}

fn count_max_cubes(mut line_deque:  VecDeque<char>) -> Result<BagOfCubes, Box<dyn Error>>{
    //struct that tracks amount of cubes necessary
    let mut cube_counter = BagOfCubes{
        red_ct: 0,
        green_ct: 0,
        blue_ct: 0,
    };

    /*
        After struggling with part 1's parsing I decided to work backwards across the 
        line which allows me to use deque better. I did this as it is easier for me to
        parse color->ct rather than ct->color as I will not have to store ct and update it
        separately from final, can just use a struct to track all 3 color counts.

        Update: turns out I will have to use insert to prepend to 'current_word'
        so it will not be faster. ohy well I tried
     */
    let mut current_word: String = "".to_string();
    while *line_deque.back().unwrap() != ':' && !line_deque.is_empty(){
        let current_letter = line_deque.pop_back().unwrap();
        if current_letter.is_numeric(){
            if current_word == "red" {
                current_word.clear();
                current_word.push(current_letter);
                //loop to get numbers with large amount of digits
                while line_deque.back().unwrap().is_numeric() {
                    current_word.insert(0,line_deque.pop_back().unwrap()); 
                }
                if cube_counter.red_ct <= current_word.parse::<i32>().unwrap() {
                    cube_counter.red_ct = current_word.parse::<i32>().unwrap();
                }
            }
            else if current_word == "blue" {
                current_word.clear();
                current_word.push(current_letter);
                //loop to get numbers with large amount of digits
                while line_deque.back().unwrap().is_numeric() {
                    current_word.insert(0,line_deque.pop_back().unwrap()); 
                }
                if cube_counter.blue_ct <= current_word.parse::<i32>().unwrap() {
                    cube_counter.blue_ct = current_word.parse::<i32>().unwrap();
                }
            }
            else if current_word == "green" {
                current_word.clear();
                current_word.push(current_letter);
                //loop to get numbers with large amount of digits
                while line_deque.back().unwrap().is_numeric() {
                    current_word.insert(0,line_deque.pop_back().unwrap()); 
                }
                if cube_counter.green_ct <= current_word.parse::<i32>().unwrap() {
                    cube_counter.green_ct = current_word.parse::<i32>().unwrap();
                }
            }
            else {
                return Err(Box::new(MyError("Invalid word in string".into())));
            }
            current_word.clear()
        } //if numeric
        else if current_letter.is_alphabetic() {
            current_word.insert(0,current_letter);
        }
        
    }

    Ok(cube_counter)
}