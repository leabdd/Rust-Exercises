mod list;
use list::{ListElem,ListHead};

fn main() {
    println!("Hello, world!");
    let mut mylist = ListHead {first: None};
    mylist.insert(5);
    mylist.insert(2);
    print!("{}",mylist)
}
  