use std::fs;
use std::error::Error;

pub fn read_doc(filename: &str) -> Result<String, Box<dyn Error>>{
    
    /*let paths = fs::read_dir(".").unwrap();

    for path in paths {
        println!("{:?}", path.unwrap().file_name());
    }
    */
   
    Ok(fs::read_to_string(&filename).unwrap())

}
struct Symbol {
    adjacents:[(i32,i32); 9],
}

impl Symbol {
    pub fn new(x:i32,y:i32)->Self{
        Self {
            adjacents: [
                (x - 1, y - 1),
                (x, y - 1),
                (x + 1, y - 1),
                (x - 1, y),
                (x, y),
                (x + 1, y),
                (x - 1, y + 1),
                (x, y + 1),
                (x + 1, y + 1),
            ],
        }
    }
}



struct Number {
    pub number : i32,
    pub positions : Vec<(i32,i32)>,
}

impl Number {
    fn is_touching_any(&self, syms : &[Symbol]) -> bool {
        syms.iter().any(|s| self.is_touching(s))
    }

    fn is_touching(&self, sym: &Symbol) -> bool {
        sym.adjacents.iter().any(|p| self.positions.contains(p))
    }
}



pub fn part_one(raw_input : String) -> Result<i32,Box<dyn Error>>{
    
    let mut symbols: Vec<Symbol> = vec![];
    let mut numbers: Vec<Number> = vec![];

    for (line_num,line) in raw_input.lines().enumerate() {
        //line num is row count or y
        // line.len() is column count or amount of items
        numbers.append(&mut get_numbers(line, line_num as i32).unwrap());

        for (x,c) in line.chars().enumerate() {
            if !c.is_numeric() && c != '.' {
                symbols.push(Symbol::new(x as i32, line_num as i32));
            }
        }

    }

    Ok(numbers.iter()
    .filter(|number| number.is_touching_any(&symbols))
    .map(|number| number.number)
    .sum::<i32>())

}

fn get_numbers(line : &str, y: i32) -> Result<Vec<Number>, Box<dyn Error>> {
    let mut temp_digit_pos: Vec<(i32,i32)> = vec![];
    let mut temp_numbers: Vec<char> = vec![];

    let mut numbers : Vec<Number> =  vec![];
    for (x,c) in line.chars().enumerate() {
        if c.is_numeric() {
            temp_digit_pos.push((x as i32, y));
            temp_numbers.push(c);
        }
        else if !temp_numbers.is_empty() {
            numbers.push(Number{
                number : String::from_iter(&temp_numbers).parse::<i32>().unwrap(),
                positions: temp_digit_pos.clone(),
            });
            temp_digit_pos.clear();
            temp_numbers.clear();
        }
    }
    if !temp_numbers.is_empty() {
        numbers.push(Number{
            number : String::from_iter(&temp_numbers).parse::<i32>().unwrap(),
            positions: temp_digit_pos.clone(),
        });
    }
    Ok(numbers)
}