mod front_of_house;

mod customer {
    use crate::front_of_house::hosting;
    use crate::back_of_house;
    
    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();

        let mut meal = back_of_house::Breakfast::summer("Rye");
        println!("The meal's toast is {}", meal.toast);
    }
}

fn deliver_oder() {}

pub mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"), 
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_oder();
    }

    pub fn cook_order() {}
}
