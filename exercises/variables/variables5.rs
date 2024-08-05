// variables5.rs
//
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a
// hint.


fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    //类似于一个语法糖，重新let这个变量就会舍弃掉原来的数据
    println!("Number plus two is : {}", number + 2);
}
