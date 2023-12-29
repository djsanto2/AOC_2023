pub mod decoder;
use decoder::*;

fn main() {
    //I am splitting this into as many modules/functions
    //As possible to learn how rust works


    //steps
    /*
     * 1. Read file (either line by line or whole)
     * 2. parse for first and last number of the line
     * 3. store and add numbers
     */

    let lines = read_doc("test.txt").unwrap();
    //codeAdder(lines);
    
    //println!("{:?}",code_adder(lines));
    println!("{:?}",code_adder_alpha_num(lines));

}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1(){
        assert!(decoder::parse_line_alpha_num("sevenine") == 79);
    }

    #[test]
    fn test2(){
        assert!(decoder::parse_line_alpha_num("8five9twothr3ees2ix") == 82);
    }

    #[test]
    fn test3(){
        assert!(decoder::parse_line_alpha_num("one") == 11);
    }

    #[test]
    fn test4(){
        assert!(decoder::parse_line_alpha_num("2one") == 21);
        assert!(decoder::parse_line_alpha_num("one2") == 12);
        assert!(decoder::parse_line_alpha_num("33") == 33);
        assert!(decoder::parse_line_alpha_num("ninetynine") == 99);
    }

   
}