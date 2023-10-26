fn main() {

    fn first_world(s:&str)->&str{
        let bytes = s.as_bytes();
        for(i,&item) in bytes.iter().enumerate(){
            if item == b' '{
                return &s[..i];
            }
        }
        &s[..]
    }



    // struct ImportantExcerpt<'a> {
    //     part: &'a str,
    // }


    // let novel = String::from("Call me Ishmael. Some years ago...");

    // let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    // let i = ImportantExcerpt { part: first_sentence };





    // let str1 = String::from("hello11");
    // let str2 = "hello";
    // let result = longest(str1.as_str(), str2);
    // println!("The longest string is {}", result);

    // fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    //     if x.len() > y.len() {
    //         x
    //     } else {
    //         y
    //     }
    // }

    // {
    //     let r;
    //     {
    //         let x = 5;
    //         r = &x;
    //     }
    //     println!("r: {}", r);
    // }

    // let x = 5;
    // let r = &x;

    // println!("r: {}", r);
}
