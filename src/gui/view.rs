extern crate gtk;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Entry, TextView};
use crate::app::EmojiBase;
use crate::app::find_emojis;

fn generate_emoji_view(search_results: Vec<String>) -> String {
    let mut result: String = String::new();
    for emoji in search_results {
        result.push_str(&emoji);
    }
    return result;
}

pub fn build_window(application: &gtk::Application) {
    let emojis : EmojiBase = crate::app::load_emojibase();
    let glade_src = include_str!("emoji_window.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("main").expect("Couldn't get main window object");
    let search_button: Button = builder.get_object("search-btn").expect("Couldn't get search-button");
    window.set_application(application);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    search_button.connect_clicked(move |_| {
        let search_box : Entry = builder.get_object("search-box").expect("Couldn't get search-box");
        let search_phrase = search_box.get_text().expect("Couldn't get text from search-box");
        let result_box : TextView = builder.get_object("emojis-view").expect("Couldn't get emojis-view");
        println!("{}", search_phrase);
        let search_results = find_emojis(&emojis, search_phrase);
        println!("{:?}", search_results);
        result_box.get_buffer().expect("Couldn't get text view buffer").set_text(&generate_emoji_view(search_results));
    });

    window.show_all();
}