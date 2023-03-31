use std::{thread_local, cell::RefCell, thread::LocalKey};

thread_local! {
    static FOO: RefCell<usize> = RefCell::new(0);
}

struct Bar {
    foo: &'static LocalKey<RefCell<usize>>
}

impl Bar {
    fn constructor() -> Bar {
        Bar {
            foo: &FOO
        }
    }
}


fn main() {
    println!("This will work for any amount of struct instances. If you want only one struct instance, then your struct itself must go in a thread local variable. The first method will not depend on an instance of the struct existing though, the second one will.");

    let bar = Bar::constructor();
    bar.foo.with(|x| println!("{x:?}") );
}