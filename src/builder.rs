#[derive(Default, Debug, Clone)]
pub struct Car {
    seats: u8,
    engine: String,
    trip_computer: Option<String>,
    gps: Option<String>,
}

pub trait Builder {
    fn reset(&mut self);
    fn set_seats(self, num_of_seats: u8) -> Self;
    fn set_engine(self, engine_name: String) -> Self;
    fn set_trip_computer(self, is_required: bool) -> Self;
    fn set_gps(self, is_required: bool) -> Self;
}

#[derive(Default)]
pub struct CarBuilder {
    car: Car,
}

impl CarBuilder {
    pub fn new() -> CarBuilder {
        CarBuilder::default()
    }

    pub fn get_product(&mut self) -> Car {
        let car = self.car.clone();
        self.reset();
        car
    }
}

impl Builder for CarBuilder {
    fn reset(&mut self) {
        *self = CarBuilder::default();
    }

    fn set_seats(mut self, num_of_seats: u8) -> Self {
        self.car.seats = num_of_seats;
        self
    }

    fn set_engine(mut self, engine_name: String) -> Self {
        self.car.engine = engine_name;
        self
    }

    fn set_trip_computer(mut self, is_required: bool) -> Self {
        if is_required {
            self.car.trip_computer = Some("Good Trip Computer".to_string());
        }
        self
    }

    fn set_gps(mut self, is_required: bool) -> Self {
        if is_required {
            self.car.gps = Some("Good GPS".to_string());
        }
        self
    }
}

pub struct Director;
impl Director {
    pub fn construct_sport_car(self, mut builder: impl Builder) {
        builder.reset();
        builder.set_seats(2)
        .set_engine("Sport Engine".to_string())
        .set_trip_computer(true)
        .set_gps(true);
    }
}
