mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {
            println!("add_to_wait_list");
        }

        fn some_function(){}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wait_list()
}



// pub mod back_of_house {
//    pub struct Breakfast {
//       pub  toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant(){
//     // let mut meal = back_of_house::Breakfast::summer("Rye");
//     let mut meal = crate::back_of_house::Breakfast::summer("Rye");

//     meal.toast = String::from("Wheat");
//     println!("I'd like {} toast please", meal.toast);
// }



// fn serve_order() {
//     println!("serve_order");
// }

// pub mod back_of_house1 {
//   pub  fn fix_incorrect_order() {
//         cook_order();
//         // 使用相对路径的方式调用
//         super::serve_order();
//         // 使用绝对路径的方式调用
//         crate::serve_order();
//     }

//     fn cook_order() {}
// }