mod factory;
use factory::{Dialog, WebDialog, WindowsDialog};

mod builder;
use builder::Builder;

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
    let car = builder::CarBuilder::new()
        .set_seats(4)
        .set_engine("V12 engine".to_string())
        .set_trip_computer(true)
        .set_gps(true)
        .get_product();
    println!("{:?}", car);
}
