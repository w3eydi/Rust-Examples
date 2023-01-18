use std::any::Any;

#[test]
fn any_trait() {
    let u32s = FavoriteThing {
        thing: Box::new(10_u32),
    };

    let floats = FavoriteThing {
        thing: Box::new(3.12_f32),
    };

    let expected_u32 = u32s.get::<u32>().unwrap();
    assert_eq!(*expected_u32, 10);
}

struct FavoriteThing {
    thing: Box<dyn Any + 'static>,
}

impl FavoriteThing {
    pub fn get<T: Any + 'static>(&self) -> Option<&T> {
        self.thing.downcast_ref()
    }
}
