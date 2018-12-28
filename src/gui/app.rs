extern crate gtk;
extern crate gio;

#[derive(Clone)]
pub struct Application {
  pub app: gtk::Application
}

pub fn init_app() -> Application {
  let application = gtk::Application::new("exchange.zack.EmojiBar", gio::ApplicationFlags::empty()).expect("Init failed");
  let app_obj = Application{app: application};
  return app_obj;
}