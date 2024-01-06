mod gardening;
use gardening::*;


fn main() {
    
    let lines = read_doc("input.txt").unwrap();

    let result_part_one : Vec<i64> = part_one(&lines.clone()).unwrap().iter()
    .map(|s| s.parse::<i64>().unwrap()).collect();
    println!("{:?}", result_part_one.iter().min().unwrap());
}


#[cfg(test)]
mod tests_part1{
    
    use super::*;
    #[test]
    fn test_file() {
        let lines = read_doc("test.txt").unwrap();
        let result_vec : Vec<i64>= part_one(&lines.clone()).unwrap().iter()
        .map(|x| x.parse::<i64>().unwrap()).collect();
        assert!(result_vec.len() ==  4);
        assert!(result_vec.iter().min() == Some(&(35 as i64)));
    }
}