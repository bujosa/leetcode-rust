mod easy;

fn main() {

    // Roman to Interger
    let s = String::from("III");
    let result = easy::roman_to_integer::roman_to_int(s);
    println!("result: {}", result);

    // Longest Common Prefix
    let strs = vec![String::from("flower"), String::from("flow"), String::from("flight")];
    let result = easy::longest_common_prefix::longest_common_prefix(strs);
    println!("result: {}", result);
    
}
