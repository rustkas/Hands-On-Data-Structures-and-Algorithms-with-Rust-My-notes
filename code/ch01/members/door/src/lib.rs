#![allow(dead_code)]
#![allow(soft_unstable)]

use std::io;

#[derive(Debug)]
struct Door {
    is_open: bool,
}
impl Door {
    fn new(is_open: bool) -> Door {
        Door { is_open: is_open }
    }
}
trait Openable {
    fn open(&mut self);
}
impl Openable for Door {
    fn open(&mut self) {
        self.is_open = true;
    }
}

fn find(_needle: u16, _haystack: Vec<u16>) -> Option<usize> {
    // find the needle in the haystack
    unimplemented!();
}
fn read_file(_path: &str) -> Result<String, io::Error> {
    // open the path as a file and read it
    unimplemented!();
}

use std::rc::Rc;
#[derive(Debug)]
struct FileName {
    name: Rc<String>,
    ext: Rc<String>,
}
fn ref_counter() {
    let name = Rc::new(String::from("main"));
    let ext = Rc::new(String::from("rs"));
    for _ in 0..3 {
        println!(
            "{:?}",
            FileName {
                name: name.clone(),
                ext: ext.clone()
            }
        );
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn open_door() {
        let mut door = Door::new(false);
        door.open();
        assert!(door.is_open);
    }

    #[test]
    fn test_find() {}

    #[test]
    fn test_ref_counter() {
        ref_counter()
    }
}

#[test]
fn shared_data() {
    use std::sync::{Arc, Mutex};
    use std::thread;

    fn shared_state() {
        let v = Arc::new(Mutex::new(vec![]));
        let handles = (0..10).map(|i| {
            let numbers = Arc::clone(&v);
            thread::spawn(move || {
                let mut vector = numbers.lock().unwrap();
                vector.push(i)
            })
        });

        for handle in handles {
            handle.join().unwrap();
        }
        println!("{:?}", *v.lock().unwrap());
    }
    shared_state();
}

pub fn my_add(a: i32, b: i32) -> i32 {
    a + b
}



#[cfg(test)]
mod tests2 {
    use super::*;
    // use test::Bencher;


    #[test]
    fn this_works() {
        assert_eq!(my_add(1, 1), 2);
    }
    #[test]
    #[should_panic(expected = "attempt to add with overflow")]
    fn this_does_not_work() {
        assert_eq!(my_add(std::i32::MAX, std::i32::MAX), 0);
    }
    
    // #[bench]
    // fn how_fast(b: &mut Bencher) {
    //     b.iter(|| my_add(42, 42))
    // }
}
