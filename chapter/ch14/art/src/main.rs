use art::{ kinds, utils };
use kinds::PrimaryColor;

fn main() {
  let red = PrimaryColor::Red;
  let yellow = PrimaryColor::Yellow;
  utils::mix(red, yellow);
}