pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}


impl AveragedCollection {
    pub fn new() -> Self{
        AveragedCollection{
            list: vec![],
            average: f64::NAN,
        }
    }
    pub fn add(&mut self, value: i32) {
        println!("Adding {}.", value);
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        println!("Removing an {:?}.", result);
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        println!("Updating average.");
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}


pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    //vector of trait objects
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing a button.");
    }
}
