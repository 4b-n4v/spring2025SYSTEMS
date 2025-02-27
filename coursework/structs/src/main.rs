#[derive(Debug)]
struct Car {
    body: String,
    year: u16,
    color: String,
}

// fn accept car, dattatype, print attributes
// fn print_attributes(car: &Car) {
//     println!("Body: {}\nYear: {}\nColor: {}",
//         car.body,
//         car.year,
//         car.color
//     );
// }

impl Car {
    fn new(b: String, y: u16, c: String) -> Self {
        Car {
            body: b,
            year: y,
            color: c,
        }
    }

    fn show_info(&self) {
        println!("{}\n{}\n{}\n", self.body, self.year, self.color);
    }
    fn change_color(&mut self, new_color: String) {
        self.color = new_color;
    }
}
fn main() {
    // let my_car = Car {
    //     body: "Sedan".to_string(),
    //     year: 2020,
    //     color: "Purple".to_string(),
    // };
    // print_attributes(&my_car);
    let mut my_car = Car::new("Sedan".to_string(), 2020, "Purple".to_string());
    my_car.show_info();
    my_car.change_color("Black".to_string());
    my_car.show_info();
    // println!("{:#?}", my_car);
}
