use add_one;
use rand;
use add_two::add;

fn main() {
    let num = 10;
    println!("{} + 1 = {}", num, add_one::add_one(num));
    // make u64
    let num = num as u64;
    println!("{} + {} = {}", num, num, add(num, num));
}
