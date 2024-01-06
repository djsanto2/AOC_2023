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
    let mut split_id: Option<usize> =None;
    let mut scores = vec![0;input.lines().count()];
    for (idx, line) in input.lines().enumerate() {
        let mut card : Vec<&str>= line.split_once(':').unwrap().1.split_whitespace().collect();

        if split_id.is_none() {split_id = card.iter().position(|&x| x == "|"); }

        card.remove(split_id.unwrap()); //removes the middle delimiter
        let mut winning_numbers : Vec<_> = card.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let play_numbers = winning_numbers.split_off(split_id.unwrap());

        let score = play_numbers.iter().map(|x| winning_numbers.contains(x) as usize).sum::<usize>();
        scores[idx] = if score == 0 {0} else {2_usize.pow((score-1) as u32)};
    }
    Ok(scores.iter().sum::<usize>() as i32)
}


pub fn part_two(input : &str) -> Result<i32, Box<dyn Error>> {
    let mut split_id: Option<usize> =None;
    let mut multiplier = vec![1;input.lines().count()];
    for (idx, line) in input.lines().enumerate() {
        let mut card : Vec<&str>= line.split_once(':').unwrap().1.split_whitespace().collect();

        if split_id.is_none() {split_id = card.iter().position(|&x| x == "|"); }

        card.remove(split_id.unwrap()); //removes the middle delimiter
        let mut winning_numbers : Vec<_> = card.iter().map(|x| x.parse::<usize>().unwrap()).collect();
        let play_numbers = winning_numbers.split_off(split_id.unwrap());
        let score = play_numbers.iter().map(|x| winning_numbers.contains(x) as usize).sum::<usize>();
        for game in idx+1..idx+1+score {
            multiplier[game] += multiplier[idx];
        }
    }
    Ok(multiplier.iter().sum::<usize>() as i32)
}