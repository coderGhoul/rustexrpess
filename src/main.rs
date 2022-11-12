fn main() {

    // let mut x  = 5;
    // // x = x + 1;

    // let x  =10;
    // println!("the number is {}",x);
    // let a = true;
    // let b:bool = false;
    // println!("{}",a^b)
    let a = "32";
    // let b:u8  =-127;
    //u是不带符号的,I是带有符号的
    // let c:f32 =1.0;
    // let d = 2.2;
    // let e = c + d;
   let b = "45".parse::<u32>().expect("Not a number");
    println!("{}",b)
}
