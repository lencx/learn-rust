fn serve_order() {}

fn fix_incorrect_order() {
	cook_order();
	self::serve_order();
}

fn cook_order() {}

pub struct Breakfast {
	pub toast: String,
	seasonal_fruit: String,
}

impl Breakfast {
	pub fn summer(toast: &str) -> Breakfast {
		Breakfast {
			toast: String::from(toast),
			seasonal_fruit: String::from("peaches"),
		}
	}
}

pub enum Appetizer {
	Soup,
	Salad,
}