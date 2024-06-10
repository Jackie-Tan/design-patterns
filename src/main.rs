mod factory;
use factory::{Dialog, WebDialog, WindowsDialog};

mod builder;
use builder::{Builder, Director};

struct Application {
    dialog: Box<dyn Dialog>,
}

impl Application {
    pub fn new() -> Application {
        // Windows or Web?
        // Get it from a config file or the environment or the parameters
        let config = "Windows";
        match config {
            "Windows" => Application {
                dialog: Box::new(WindowsDialog {}),
            },
            "Web" => Application {
                dialog: Box::new(WebDialog {}),
            },
            _ => {
                println!("Did not indicate the config. Exiting...");
                std::process::exit(1);
            }
        }
    }
}
fn main() {
    /* Factory method */
    let app = Application::new();
    app.dialog.render();

    /* Builder method */
    // The client could control the builders directly.
    let car = builder::CarBuilder::new()
        .set_seats(4)
        .set_engine("V12 engine".to_string())
        .set_trip_computer(true)
        .set_gps(true)
        .get_product();
    println!("{:?}", car);

    // The client could also use a director (if available)
    // for executing the building steps in a particular sequence.
    let mut car_builder = builder::CarBuilder::new();
    Director::construct_sport_car(&mut car_builder);
    let sport_car = car_builder.get_product();
    println!("{:?}", sport_car);
}
