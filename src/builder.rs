#[derive(Debug)]
pub struct Car {
    seats: u8,
    engine: String,
    trip_computer: Option<String>,
    gps: Option<String>,
}

pub trait Builder {
    fn set_seats(&mut self, num_of_seats: u8) -> &mut Self;
    fn set_engine(&mut self, engine_name: String) -> &mut Self;
    fn set_trip_computer(&mut self, is_required: bool) -> &mut Self;
    fn set_gps(&mut self, is_required: bool) -> &mut Self;
}

pub struct CarBuilder(Car);

impl CarBuilder {
    pub fn new() -> Self {
        CarBuilder(Car {
            seats: u8::default(),
            engine: String::default(),
            trip_computer: None,
            gps: None,
        })
    }

    pub fn build(&self) -> Car {
        Car {
            seats: self.0.seats,
            engine: self.0.engine.clone(),
            trip_computer: self.0.trip_computer.clone(),
            gps: self.0.gps.clone(),
        }
    }
}

impl Builder for CarBuilder {
    fn set_seats(&mut self, num_of_seats: u8) -> &mut Self {
        self.0.seats = num_of_seats;
        self
    }

    fn set_engine(&mut self, engine_name: String) -> &mut Self {
        self.0.engine = engine_name;
        self
    }

    fn set_trip_computer(&mut self, is_required: bool) -> &mut Self {
        if is_required {
            self.0.trip_computer = Some("Good Trip Computer".to_string());
        }
        self
    }

    fn set_gps(&mut self, is_required: bool) -> &mut Self {
        if is_required {
            self.0.gps = Some("Good GPS".to_string());
        }
        self
    }
}

pub struct Director;
impl Director {
    pub fn construct_sport_car(builder: &mut impl Builder) {
        builder
            .set_seats(2)
            .set_engine("Sport Engine".to_string())
            .set_trip_computer(true)
            .set_gps(true);
    }
}
