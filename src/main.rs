// fn main() {

//     // let mut x  = 5;
//     // // x = x + 1;

//     // let x  =10;
//     // println!("the number is {}",x);
//     // let a = true;
//     // let b:bool = false;
//     // println!("{}",a^b)
//     let a = "32";
//     // let b:u8  =-127;
//     //u是不带符号的,I是带有符号的
//     // let c:f32 =1.0;
//     // let d = 2.2;
//     // let e = c + d;
// //    let b = "45".parse::<u32>().expect("Not a number");
// //     println!("{}",b)

// //复合类型 ：两种原始的符合类型：元组和数组
//  let  tup:(i32,f64,u8) =  (600,64.2,1);
//  //与JavaScript用（）解构
//  let (x,y,z) = tup;
//  println!("{}",x);
//  println!("{}",y);
//  println!("{}",z);

//  let tup_one = tup.0;
//  println!("{}",tup_one);

//  //数组,分配在堆栈上而不是在堆上
//  let arr:[i32;5] = [1,2,3,4,5];
//  //默认生成值
//  let arr1 = [3;5];
//  let first = arr1[0];
//  //越界
//  let index = arr1[100];
//  println!("{}",first);
//  //直接报错ndex out of bounds: the length is 
// // 5 but the index is 100
//  println!("{}",index);

// }

// fn main(){
//     println!("hello world");
//     another_function(3);

//     //语句和表达式
//     //错误案例
//     // let x = (let y = 6);
//     let y = {
//         let x = 3;
//         x + 1
//     };
//     println!("The value of y is: {y}");
// }
// //调用方法
// fn another_function(x:i32){
//     //两种写法都可以
//     // println!("Anoter_function{}",x);
//     println!("Anoter_function{x}");
//     let y =  five();
//     println!("{y}")
// }

// fn five()-> i32{
//     return 1
// }

// fn main(){
//     let number =3;
//     if number > 5 {
//         println!("condition was true");
//     } else {
//         println!("condition was false");
//     }

//     if number % 4 == 0 {
//         println!("number is divisble by 4");
//     }else if number % 3 == 0{
//         println!("number is divisable bu3");
//     }else if number % 2 ==0 {
//         println!("number is divisable by 2 ");
//     }else {
//         println!("number is not divisible by 2 3 4");
//     }

//     //if在let语句单独使用
//     let condition = true;
//     let number_2 = if condition {5} else {6};
//     println!("{}",number_2);
// }

//循环
fn main(){
    
    // loop{
    //     println!("1");
    // }
    //从循环种返回值
    // let mut counter = 0;
    // let result = loop{
    //     counter += 1;
    //     if counter ==10{
    //         break counter *2
    //     } 
    // };
    // println!("{}",result)

    //循环标签以消除多个循环之间的歧义
    // let mut count = 0;
    // 'counting_up:loop {
    //     println!("count = {count}");
    //     let mut remaining =10;
    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -=1;
    //     }
    //     count +=1;
    // }
    // println!("End count = {count}")
    //条件while语句
    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -=1
    }
    println!("LIFTOFF!")
}