use std::collections::HashMap;


fn main() {

    let mut scopes = HashMap::new();
    scopes.insert(String::from("blue"),10);
    scopes.insert(String::from("red"),2);


    





    /*
        let hello = "小明在学习";
        let s = &hello[0..6];
        println!("s = {}", s);

        for b in hello.as_bytes() {
            print!("{} ", b);
        }

        for c in hello.chars() {
            print!("{} ", c);
        }
    */

    /*
        // let s1 = String::from("hello");
        // let s2 = String::from("world");
        // let s3 = s1 + &s2;
        // println!("s3 = {}", s3);


        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        // let s4 = s1+ "-" + &s2 + "-" + &s3;
        // println!("s4 = {}", s1);


        let s = format!("{}-{}-{}",s1,s2,s3);
        println!("s = {}", s);

    */

    /*
        let s = String::from("hello world");
        // let  hello = &s[0..5];
        // let world = &s[6..11];
        let hello = &s[..5];
        let world = &s[6..];
        let whole = &s[..];

        println!("hello = {}, world = {},whole = {}", hello, world,whole);

    */

    /*   fn first_world(s:&String)->usize{
            let bytes = s.as_bytes();
            for(i,&item) in bytes.iter().enumerate(){
                if item==b' ' {
                    return i
                }
            }
            s.len()
         }

         let mut s = String::from("hello world");
         let word_index = first_world(&s);

         s.clear();
         println!("wordIndex = {}, s = {}", word_index,s);
    */

    /*
        let r =dangling();


        fn dangling()->&String{
            let s = String::from("hello");
            s
        }


        // let mut s = String::from("hello");
        // let s2 = &s;
        // let s3 = &s;
        // let s1 = &mut s;

        // println!("s1 = {}, s2 = {}, s3 = {}", s1, s2, s3);
        // let mut s = String::from("hello");
        // let r1 = &s;
        // let r2 = &s;
        // let r3 = &mut s;

        // println!("{}, {}, {}", r1, r2, r3);

        // let mut s = String::from("hello");
        // {
        //     let s1 = &mut s;
        // }
        // let s2:&mut String = &mut s;
        // println!(" s2 = {}",  s2);
    */

    /*
        let mut s1 = String::from("hello");
       let length = calculate_length(&mut s1);
        println!("s1 = {},len = {}", s1,length); // error


        fn calculate_length(s:&mut String)->usize{
            s.push_str("world");
            s.len()
        }
    */

    /*    fn give_ownership() -> String{
            let s = String::from("hello");
            s
        }

        fn take_and_give_back(s:String) -> String{
            s
        }

        let _s1 = give_ownership();
        let _s2 = String::from("hello");
        let _s3 = take_and_give_back(_s2.clone());

        println!("s3 = {},s2 = {} , s1 = {}", _s3,_s2,_s1);
    */

    /*let mut s: String = String::from("hello");
    s.push_str("world");
    println!("s = {}", s);

    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

    let x = 5;
    let y = x;
    println!("x = {}, y = {}", x, y);

    let s = String::from("hello");
    takes_ownership(&s);
    println!("s = {}", s); // error

    fn takes_ownership (s:&str){
        println!("s {}",s)
    }*/

    /*
        let x = 5;
        let y =x;
        println!("x = {}, y = {}", x, y);
     // rust 核心观点 所有权 s1 交出所有权后就不能使用
        let s1 = String::from("hello");
        let s2 = s1;
        println!("s2 = {}", s2);
        println!("s1 = {}", s1); // error



        let s = "&str";
        let mut s1:String = String::from("hello");
        s1.push_str("world");
        println!("s1 = {}", s1);
    */
    /*
    let mut counter = 0;
    let res = loop{
        counter += 1;
        if counter==10 {
            break counter;
        }
    };
    println!("res = {}", res);


    let mut index = 0;
    while index < 10{
        index+=1;

    }
    println!("index = {}", index);

    let arr = [1,2,3,4,5];
    for num in arr.iter(){
        println!("num = {}", num);
    }

    for num in (1..4).rev(){
        println!("num = {}", num);
    }

    */
    /*
        let condition = false;
        let num = if condition{5}else{"6".parse().unwrap()}; // 5
        println!("num = {}", num);

        let res = add(3,2);
        println!("res = {}", res);
    */
    /*

       // println!("Hello, world!");

       // let mut x:f32 = 127.43;
       // x = x - 100.00;
       // println!("x = {}", x);

       // let b : bool = true;
       // let b= false;
       // println!("b = {}", b);


       // let mut c = "a";
       // c = "b";
       // println!("c = {}", c);
       // let mut c1 = 'd';
       // c1 = 'e';
       // println!("c1 = {}", c1);


       // let tup:(i32, f64, u8) = (500, 6.4, 1);
       // let (x, y, z) = tup;
       // println!("x = {}, y = {}, z = {}", x, y, z);


       // let arr:[i32;5] = [1,2,3,4,5];
       // println!("arr[0] = {}", arr[0]);

       // let arr1 = [3;5];
       // // 打印arr1数组
       // println!("arr1 = {:?}", arr1);
       // println!("arr1[0] = {}", arr1[0]);
    */
}

// fn add(x:i32,y:i32)->i32{
//     x-y
// }
