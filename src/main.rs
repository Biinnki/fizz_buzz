fn main() {
    let x = fizz_buzz(3);
    println!("{:?}", x);
}

fn fizz_buzz(n: i32) -> Vec<String> {

    let mut list: Vec<String> = Vec::new();

    for x in 1..=n {

        if x % 3 == 0 && x % 5 == 0 {
            list.push("FizzBuzz".to_string());
        }
        else if x % 3 == 0 {
            list.push("Fizz".to_string());
        }
        else if x % 5 == 0 {
            list.push("Buzz".to_string());
        }
        else {
            list.push(x.to_string());
        }
    }

    return list;
}