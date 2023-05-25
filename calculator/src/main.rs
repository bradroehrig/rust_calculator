fn main() {

    let a = [1,2,3,4,5];
    let b = ["hi","hi","hi","hi","hi"];
    let mut index = 0;
    let mut hi = 0;

    while index<5 {
        println!("{}",a[index]);
        index = index + 1;
    }

    while hi<5 {
        println!("{}",b[hi]);
        hi = hi + 1;
    }
}