use std::fs;
use std::error::Error;

struct almanac_map {
    beginnings: Vec<String>,
    destinations: Vec<String>,
    ranges: Vec<String>,
}

impl almanac_map {
    pub fn new() -> Self {
        Self {
            beginnings: Vec::new(),
            destinations: Vec::new(),
            ranges: Vec::new(),
        }
    }

    pub fn get_destinations(&self, input_ids : Vec<String>) -> Result<Vec<String>, Box<dyn Error>> {
        let mut output_ids: Vec<String>= vec![];
        let mut number :i64;
        let mut id_num :i64;
        let mut range_number:i64;
        for id in input_ids {
            let mut id_found = false;
            for (begin_idx, begin) in self.beginnings.iter().enumerate() {
                number = begin.parse::<i64>().unwrap();
                id_num = id.parse::<i64>().unwrap();
                range_number = self.ranges[begin_idx].parse::<i64>().unwrap();
                if id_num >= number 
                &&  id_num <= (number + range_number-1){
                    output_ids.push((self.destinations[begin_idx].parse::<i64>().unwrap() + (id_num-number)).to_string());
                    id_found = true;
                }
            }
            if !id_found {
                output_ids.push(id.clone());
            }
        }
        
        Ok(output_ids)
    }

    pub fn push_numbers(&mut self, numbers : Vec<&str>) {
        self.beginnings.push(numbers[1].to_string());
        self.destinations.push(numbers[0].to_string());
        self.ranges.push(numbers[2].to_string())
    }

    pub fn clone(&self) -> almanac_map {
        let mut out_map = almanac_map::new();
        out_map.beginnings = self.beginnings.to_vec();
        out_map.destinations = self.destinations.to_vec();
        out_map.ranges = self.ranges.to_vec();
        out_map
    }

}


pub fn read_doc(filename: &str) -> Result<String, Box<dyn Error>>{
    
    Ok(fs::read_to_string(filename).unwrap())

}

//We need to parse the input into several sections 
pub fn part_one(input: &str) -> Result<Vec<String>, Box<dyn Error>>{
    //get the seeds from the file and store them in start
    let seeds : Vec<String> = input.lines().next().unwrap().split_once(':').unwrap().1.split_whitespace()
    .map(|s| s.to_string()).collect();
    //let mut alma_map :almanac_map = almanac_map::new();
    let mut destinations: Vec<String> = vec![];
    let mut beginnings: Vec<String> = vec![];
    let mut ranges: Vec<String> = vec![];
    let mut transitions:Vec<String> = vec![];
    let mut almanac_maps :Vec<almanac_map> = vec![];
    let mut seeds_used = false;
    for (idx,line) in input.lines().enumerate() {
        
        //check if new map
        if line.contains(':') {
            //follow map
            if !destinations.is_empty() && !ranges.is_empty() && !beginnings.is_empty() {
                almanac_maps.push(almanac_map{
                    beginnings: beginnings.clone(),
                    destinations: destinations.clone(),
                    ranges: ranges.clone(),
                });
                if !seeds_used {
                    transitions = almanac_maps.last().unwrap().get_destinations(seeds.clone()).unwrap();
                    seeds_used = true;
                }
                else {
                    transitions = almanac_maps.last().unwrap().get_destinations(transitions.clone()).unwrap();
                }
                destinations.clear();
                beginnings.clear();
                ranges.clear();
            }
            
        }
        else if !line.is_empty() {
            let numbers : Vec<&str> = line.split_whitespace().collect();
            destinations.push(numbers[0].to_string());
            beginnings.push(numbers[1].to_string());
            ranges.push(numbers[2].to_string());
        }
    }
    
    Ok(transitions)
}


