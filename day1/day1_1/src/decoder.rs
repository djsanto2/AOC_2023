use std::fs;
use std::error::Error;
use std::collections::{VecDeque, HashMap};

pub fn read_doc(filename: &str) -> Result<Vec<String>, Box<dyn Error>>{
    
    /*let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
    */
   
    Ok(fs::read_to_string(filename).unwrap().lines().into_iter().map(|x: &str| x.to_string()).collect())

}


pub fn code_adder(lines: Vec<String>) -> i32{

    let mut total : i32 = 0;

    for line in lines{
        total += parse_line(&line); //how do I check to only add 
    }
    total

}

pub fn code_adder_alpha_num(lines: Vec<String>) -> i32{

    let mut total : i32 = 0;

    for line in lines{
        total += parse_line_alpha_num(&line); //how do I check to only add 
    }
    total

}


fn parse_line(line: &str) -> i32{
    //how do I handle if the line does not
    //have any numbers and just return 0
    let mut first : char = '~';
    let mut last : char = '~';

    let mut check_first: bool = true;

    for c in line.chars(){
        if c.is_digit(10) {
            if check_first {
                first = c;
                check_first = false;
            }
            last = c;
        }
    }
    (first.to_string()+&last.to_string()).parse::<i32>().unwrap()

    
}


pub fn parse_line_alpha_num(line: &str) -> i32{
    let threes = HashMap::from([("one".to_string(), '1'),
    ("two".to_string(), '2'), ("six".to_string(), '6')]);
    let fours = HashMap::from([("four".to_string(), '4'),
    ("five".to_string() , '5'),  ("nine".to_string(), '9')]);
    let fives = HashMap::from([("three".to_string(), '3'),
    ("seven".to_string(), '7'), ("eight".to_string(), '8')]);
    //let mut word : String;

    let mut first: char = '0';
    let mut last: char = '0';
    let mut first_check = true;
    let mut line_deque: VecDeque<char> = VecDeque::new();
    
    
    for c in line.chars() {
        line_deque.push_front(c);
    }
    while !line_deque.is_empty(){
        let current_letter = line_deque.pop_back().unwrap();
        if current_letter.is_numeric(){
            if first_check {
                first = current_letter;
                first_check = false;
            }
            last = current_letter;
        }
        else if line_deque.len()+1 >=3{
            let mut current_word = String::from("");
            current_word.push(current_letter);
            current_word.push(*line_deque.get(line_deque.len()-1).unwrap());
            current_word.push(*line_deque.get(line_deque.len()-2).unwrap());
            //if current word is in threes - do the set first last
            //else add 4 check fours - do set first last
            //else check "eight vs 5"
            if threes.contains_key(&current_word){
                if first_check {
                    first = *threes.get(&current_word).unwrap();
                    first_check = false;
                }
                last = *threes.get(&current_word).unwrap();
            }
            else if line_deque.len()+1 >= 4{
                current_word.push(*line_deque.get(line_deque.len()-3).unwrap());
                if fours.contains_key(&current_word){
                    if first_check {
                        first = *fours.get(&current_word).unwrap();
                        first_check = false;
                    }
                    last = *fours.get(&current_word).unwrap();
                }
                else if line_deque.len()+1 >= 5{
                    current_word.push(*line_deque.get(line_deque.len()-4).unwrap());
                    if fives.contains_key(&current_word){
                        if first_check {
                            first = *fives.get(&current_word).unwrap();
                            first_check = false;
                        }
                        last = *fives.get(&current_word).unwrap();
                    }
                }
            }
        }
    }
    (first.to_string()+&last.to_string()).parse::<i32>().unwrap()

}