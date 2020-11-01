fn main() {
    let mut a: [i32; 4] = [1, 2, 3 ,5];
    a[2] = 7;
    println!("a is {:?}", a);
    a = [5,5,8,8];
    println!("a is {:?}", a);
}
