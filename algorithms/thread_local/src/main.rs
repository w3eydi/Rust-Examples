use std::{thread_local, cell::RefCell};

struct Foo {}

impl Foo {
    thread_local! {
        static FOO: RefCell<u32> = RefCell::new(0);
    }
}

fn main() {
    Foo::FOO.with(|x| {
        let value = *x.borrow() + 1;
        x.replace(value);
        println!("{}", x.borrow());
    });

    println!("Also you can check examples folder. It is containing other ->thread_local<- examples.");
    println!("Type -> cargo run --example thread_local2")
}
