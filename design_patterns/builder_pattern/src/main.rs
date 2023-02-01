#[derive(Debug)]
struct Pizza {
    dough: String,
    sauce: String,
    toppings: Vec<String>
}

struct PizzaBuilder {
    dough: String,
    sauce: String,
    toppings: Vec<String>
}

impl PizzaBuilder {
    fn new() -> PizzaBuilder {
        PizzaBuilder {
            dough: "".to_string(),
            sauce: "".to_string(),
            toppings: vec![]
        }
    }

    fn with_dough(mut self, dough: &str) -> PizzaBuilder {
        self.dough = dough.to_string();
        self
    }

    fn with_sauce(mut self, sauce: &str) -> PizzaBuilder {
        self.sauce = sauce.to_string();
        self
    }

    fn with_topping(mut self, topping: &str) -> PizzaBuilder {
        self.toppings.push(topping.to_string());
        self
    }

    fn build(self) -> Pizza {
        Pizza {
            dough: self.dough,
            sauce: self.sauce,
            toppings: self.toppings
        }
    }
}

fn main() {
    let pizza = PizzaBuilder::new()
        .with_dough("thin crust")
        .with_sauce("tomato")
        .with_topping("mozarella cheese")
        .with_topping("roasting")
        .with_topping("sausage")
        .build();
    //dbg!(pizza);
}
