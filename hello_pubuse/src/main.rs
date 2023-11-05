// use hello_pubuse::kinds::PrimaryColor;
// use hello_pubuse::utils::mix;
use hello_pubuse::PrimaryColor;
use hello_pubuse::mix;
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
