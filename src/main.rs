mod easy;

fn main() {

    // Roman to Interger
    let s = String::from("III");
    let result = easy::roman_to_integer::roman_to_int(s);
    println!("result: {}", result);
}
