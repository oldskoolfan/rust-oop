use oop::gui::{Draw, Screen, Button};

struct SelectBox {
  width: u32,
  height: u32,
  options: Vec<String>,
}

impl Draw for SelectBox {
  fn draw(&self) {
      // code to actually draw a select box
      println!(
        "drawing select box of width: {0} height: {1} options: {2:?}",
        self.width,
        self.height,
        self.options,
      );
  }
}

fn main() {
    println!("Hello, world!");

    let screen = Screen {
      components: vec![
        Box::new(SelectBox {
          width: 75,
          height: 10,
          options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
          ]
        }),
        Box::new(Button {
          width: 50,
          height: 10,
          label: String::from("OK"),
        }),
      ],
    };

    screen.run();
}
