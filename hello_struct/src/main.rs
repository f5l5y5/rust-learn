fn main() {

    impl Rectangle{
        fn area(&self)->u32{
            self.width * self.height
        }
        fn square(size:u32)->Rectangle{
            Rectangle{
                width:size,
                height:size,
            }
        }
    }




    // fn area(width:u32,height:u32)->u32{
    //     width * height
    // }


    // let width  =30;
    // let height = 50;
    // println!("area = {}",area(width,height));


    // let rect = (30,50);
    // println!("area = {}",area(rect.0,rect.1));

    #[derive(Debug)]
    struct Rectangle{
        width:u32,
        height:u32,
    }

    let rect = Rectangle{
        width:30,
        height:50,
    };



    fn area(rect:&Rectangle)->u32{
        rect.width * rect.height
    }


    println!("area = {},rect=>{:?},{:#?}",area(&rect),rect,rect);

    println!("area = {}",rect.area());

    let s = Rectangle::square(30);
    println!("s = {:#?}",s);


/* 
    struct Color(u32,u32,u32);
    let color = Color(0,1,2);
    println!("color = {},{},{}",color.0,color.1,color.2);


    struct User{
        username: String,
        email: &str,
        sign_in_count: u64,
        active: bool,
    }

    let mut user = User{
        username: String::from("jack"),
        email : "qq.com",
        sign_in_count: 1,
        active: true,
    };
    user.active = false;

    user.email = "gmail.com";

*/

 /*    
    println!("Hello, world!");
    struct User{
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    let mut user = User{
        username: String::from("jack"),
        email : String::from("12@qq.com"),
        sign_in_count: 1,
        active: true,
    };
    user.active = false;

    fn build_user(email:String,username:String)->User{
        User{
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
    
    let user2 = User{
        username:String::from("mark"),
        email:String::from("gmail"),
        active:false,
        ..user
    };

    println!("user2 = {}",user2.username);
*/


}
