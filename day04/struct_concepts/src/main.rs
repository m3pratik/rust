// Create a struct called Car with the fields: mpg, color, and top_speed. Once the struct is created, implement the following methods: set_mpg, set_color, and set_top_speed. Once you have created these methods, create a car, provide it default values, and then set the mpg, color, and top speed and then print them out.

struct Car {
    mpg: String,
    color: String,
    top_speed: i32,
}

impl Car {

    fn set_mpg(&mut self, new_mpg: String) -> () {
        self.mpg = new_mpg
    }

    fn set_color(&mut self, new_color: String) -> () {
        self.color = new_color
    }

    fn set_top_speed(&mut self, new_top_speed: i32) -> () {
        self.top_speed = new_top_speed
    }


}

fn main() {

    let mut car1 = Car {mpg: String::from("Maruti"), color: String::from("White"), top_speed: 220};
    car1.set_mpg(String::from("Honda"));
    car1.set_color(String::from("Black"));
    car1.set_top_speed(220);
    println!("{} {} {}", car1.mpg, car1.color, car1.top_speed);

}
