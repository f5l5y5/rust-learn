pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn can_hold(&self,other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


pub fn greeting(name:&str)->String{
    format!("Hello {}!",name)
}

pub struct Guess {
    value:u32
}

impl Guess {
    pub fn new (value:u32)->Guess{
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.",value);
        }

        Guess { value }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }


    #[test]
    fn larger_can_hold_smaller(){
        let larger = Rectangle { width: 8, height: 7};
        let smaller = Rectangle { width: 15, height: 1};
        assert!(larger.can_hold(&smaller),);
    }


    #[test]
    fn greeting_contains_name(){
        let result = greeting("Carol");
        assert!(result.contains("Carol"),"Greeting didn't contain me,value was '{}'",result);
    }

    #[test]
    #[should_panic]
    fn greater_than_100(){
        Guess::new(20);
    }
}
