use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

#[test]
fn type_id() {
    let mut components: HashMap<TypeId, Vec<Box<dyn Any + 'static>>> = HashMap::new();
    let health: u32 = 100;
    let health_typeid = TypeId::of::<u32>();
    components.insert(health_typeid, vec![Box::new(health)]);

    let speed = Speed(150);
    let type_id_speed = speed.type_id();
    components.insert(type_id_speed, vec![Box::new(speed)]);

    for (_component_typeid, component_value) in components {
        let type_id = component_value[0].type_id();
        dbg!(type_id);
    }
}

struct Speed(u32);
