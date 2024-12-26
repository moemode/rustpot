

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let num: &mut i32 = &mut v[2];
    let num2: &i32 = num;
    println!("{}", num);
    println!("{}", num2);
    *num = 4;
    println!("{}", num2);
}