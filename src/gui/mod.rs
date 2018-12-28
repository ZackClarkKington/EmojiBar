extern crate gtk;
extern crate gio;
use crate::gui::gio::ApplicationExt;
use crate::gui::gio::ApplicationExtManual;

pub use self::app::*;
use std::env::args;
pub mod app;

pub use self::view::*;
pub mod view;

pub fn main() {
  let application = self::init_app();
  application.app.connect_startup(move |app| {
    self::build_window(app);
  });

  application.app.connect_activate(|_| {});
  application.app.run(&args().collect::<Vec<_>>());
  gtk::main();
}