use std::rc::Rc;
//
//struct App{
//    base_url: Option<String>
//}
//
//impl App {
//    fn new() -> App {
//        App{
//            base_url: None,
//        }
//    }
//    fn get_base_url(&mut self) -> String {
//        match self.base_url {
//            Some(url) => url.clone(),
//            None => {
//                let url = "test".to_string();
//                self.base_url = Some(url);
//                self.base_url.unwrap().clone()
//            }
//        }
//    }
//}

//fn largest<T: PartialOrd>(list:&[T]) -> &T {
//    let mut largest = &list[0];
//    for (i, &ref item) in list.iter().enumerate() {
//        if &&item > &&largest {
//            largest = &list[i];
//        }
//    }
//    largest
//}

//fn longest<'a>(list: &'a[&str]) -> &'a str {
//    let mut long = &list[0];
//    for (i, &ref item) in list.iter().enumerate() {
//        if item.len() > long.len() {
//            long = &list[i];
//        }
//    }
//    long
//}

fn main(){

    //let mut app = App::new();

//    let number_list = vec![34, 50, 25, 100];
//
//
//    println!("base_url: {}", largest(&number_list))

    //let ss = vec!["a", "bbb", "cc", "dddd", "ee"];
    let mut s = Rc::new(String::from("123"));
    println!("s: {:?} : {}", &s as *const _, s);
    {
        s =  Rc::new(String::from("234"));
        //println!("a: {:?} : {}", &a as *const _, &mut s);
    }
    let b = &s;
    println!("b: {:?} : {}", &b as *const _, s);
}