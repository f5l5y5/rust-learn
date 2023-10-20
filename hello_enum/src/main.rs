fn main() {


    let v = Some(0u8);

    match v {
        Some(val)=>println!("val: {:?}", val),
        None=>println!("None"),
    }


    if let Some(3) = v {
        println!("three");
    } else {
        println!("others");
    }


    // 字面值匹配
    let x = 0;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        _ => println!("other"),
    }
    // 变量匹配
    match x {
        e => println!("e: {:?}", e),
    }

    // 通配符匹配
    match x {
        _ => println!("other"),
    }
    // 范围匹配
    match x {
        1..=5 => println!("one to five"),
        _ => println!("other"),
    }
    // 引用匹配
    let s = 1;
    let k = &s;
    match k {
        &val => println!("val is {}", val),
    }
    // 结构体匹配
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 1, y: 2 };

    match point {
        Point { x, y: 0 } => println!("x: {}, y: 0", x),
        Point { x: 0, y } => println!("x: 0, y: {}", y),
        Point { x, y } => println!("x: {}, y: {}", x, y),
    }

    // 元组匹配
    let pair = (0, -2);
    match pair {
        (0, y) => println!("x: 0, y: {}", y),
        (x, 0) => println!("x: {}, y: 0", x),
        _ => println!("other"),
    }

    // 枚举匹配
    enum Color {
        Red,
        Green,
        Blue,
        RGB(u32, u32, u32),
    }

    let color = Color::RGB(1, 2, 3);

    match color {
        Color::Red => println!("red"),
        Color::Green => println!("green"),
        Color::Blue => println!("blue"),
        Color::RGB(r, g, b) => println!("r: {}, g: {}, b: {}", r, g, b),
    }

    // 数组和切片模式
    let arr = [1, 2, 3];

    match arr {
        [a, b, c] => println!("a: {}, b: {}, c: {}", a, b, c),
        [a, ..] => println!("a: {}", a),
        [.., a] => println!("a: {}", a),
        [1, _, _] => println!("1,_,_"),
        _ => println!("other"),
    }

    // 守卫模式
    let guard = (1, 2);
    match guard {
        (x, y) if x == y => println!("equal"),
        (x, y) if x + y == 0 => println!("sum 0"),
        (x, _) if x % 2 == 1 => println!("x is odd"),
        _ => println!("No correlation..."),
    }

    // @ 捕获模式
    let some_value = 5;

    match some_value {
        e @ 1..=5 => println!("got a range element {}", e),
        _ => (),
    }

    // ? 操作符模式
    let optional = Some(7);
    match optional {
        Some(i) => println!("This is a really long string and `{:?}`", i),
        _ => {}
    }
    // ref 模式
    let x = 5;
    match x {
        ref r => println!("Got a reference to {}", r),
    }
    //mut 模式
    let mut x = 5;
    match x {
        ref mut mr => println!("Got a mutable reference to {}", mr),
    }

    /*
    // enum Option<T> {
    //     Some(T),
    //     None,
    // }

    let mut sum = 0;
    let some_number = Some(5);

    sum = some_number.unwrap() + 5; //  unwrap() 会panic
    println!("sum0: {:?}", sum);

    sum = match some_number {
        Some(x) => x + 10,
        None => 0,
    };
    println!("sum1: {:?}", sum);

    sum = if let Some(x) = some_number { x + 15 } else { 0 };
    println!("sum2: {:?}", sum);

    // let some_number:Option<i32> = None;

    // println!("some_number: {:?}, some_string: {:?}", some_number, some_string);
    */
    /*
        #[derive(Debug)]
        enum Message{
            Quit,
            Move{x:i32,y:i32},
            Write(String),
            ChangeColor(i32,i32,i32),
        }
        impl Message{
            fn call(&self){
                println!("call: {:?}", self);
            }
        }

        let quit = Message::Quit;
        let write = Message::Write(String::from("hello"));
        let move_msg = Message::Move{x:1,y:2};
        let change_color = Message::ChangeColor(1,2,3);

        quit.call();
        write.call();
        move_msg.call();
        change_color.call();
    */
    /*
        #[derive(Debug)]
        enum IpAddrKind{
            V4,
            V6,
            Color(String),
            IP(u32,u32,u32,u32 ),

        }
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        println!("four: {:?}, six: {:?}", four, six);
        route(four);
        route(six);
        route(IpAddrKind::V6);


        fn route(ip_kind:IpAddrKind){
            println!("ip_kind: {:?}", ip_kind);
        }

        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }

        let loopback = IpAddr{
            kind: IpAddrKind::V4,
            address: String::from("dddd"),
        };

        println!("loopback: {:?}", loopback);
    */
}
