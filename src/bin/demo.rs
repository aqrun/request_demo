use std::rc::Rc;

struct st {
    age: i32
}

fn main(){

//    let mut a = st{age: 14};
//    a.age = 23;
//    println!("{}", a.age)

    let mut a = Rc::new(st{age:3});
//    {
//        let mut b = a.clone();
//        //b.age = 5;
//        println!("count a 1:{}", Rc::strong_count(&a));
//        println!("b.age: {}", b.age)
//    }
//    println!("count a 2:{}",Rc::strong_count(&a));
    a.age = 10;
    println!("a.age: {}", a.age)
}