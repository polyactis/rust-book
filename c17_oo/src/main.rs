use c17_oo::{Button, Screen, Draw, AveragedCollection};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw a select box
        println!("Drawing a SelectBox w={}, h={}, o={:?}.", 
            self.width, self.height, self.options);

    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
    let mut avg_collector = AveragedCollection::new();
    println!("average is {}.", avg_collector.average());
    avg_collector.add(3);
    avg_collector.add(100);

    avg_collector.add(17);

    println!("average is {}.", avg_collector.average());
    avg_collector.remove();
    println!("average is {}.", avg_collector.average());
    avg_collector.remove();
    avg_collector.remove();
    avg_collector.remove();
    avg_collector.remove();
    println!("average is {}.", avg_collector.average());

}