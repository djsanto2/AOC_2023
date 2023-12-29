pub mod decoder;
use decoder::*;

#[cfg(test)]
mod tests() {
    let test1 = "sevenine".to_string();
    assert!(decoder::parse_line_alpha_num(test1) == 79);

    let test2 = "8five9twothr3ees2ix".to_string();
    assert!(decoder::parse_line_alpha_num(test2) == 82);
}