pub trait Draw {
  fn draw(&self);
}

pub struct Button {
  pub width: u32,
  pub height: u32,
  pub label: String,
}

impl Draw for Button {
  fn draw(&self) {
      // code to actually draw button
      println!(
        "drawing button of width: {0} height: {1} label: {2}",
        self.width,
        self.height,
        self.label,
      );
  }
}

pub struct Screen {
  pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
  pub fn run(&self) {
    for component in self.components.iter() {
      component.draw();
    }
  }
}
