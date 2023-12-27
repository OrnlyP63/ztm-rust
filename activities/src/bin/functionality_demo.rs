struct Temperature {
    degree_f: f64,
}

impl Temperature {

    fn freezing() -> Self {
        Self { degree_f: 32.0 }
    }

    fn change_temp(&mut self) {
        self.degree_f = 32.0;
    }

    fn _show_temp(temp: &Temperature) {
        println!("{:?} degree F :associated function", temp.degree_f);
    }

    fn show_temp(&self) {
        println!("{:?} degree F :method", self.degree_f);
    }
}

fn main() {
    let mut hot = Temperature{ degree_f: 99.0};
    Temperature::_show_temp(&hot);
    hot.show_temp();
    hot.change_temp();
    hot.show_temp();

    let cold = Temperature::freezing();
    cold.show_temp()
}