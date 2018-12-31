extern crate gtk;
extern crate pango;
use gtk::prelude::*;
use gtk::{ApplicationWindow, Builder, Button, Entry, Grid, Label};
use crate::app::EmojiBase;
use crate::app::find_emojis;

fn generate_emoji_view(search_results: Vec<String>, grid: &Grid){
    let mut index = 0;
    let mut row = 0;
    let attr_list = pango::AttrList::new();
    let mut attr = pango::Attribute::new_scale(5.0).expect("Couldn't create new scale attribute");
    attr_list.insert(attr);
    while row < (search_results.len() / 5 + 1) {
        for column in 0..5 {
            if(index < search_results.len()){
                let emoji = &search_results[index];
                let label : Label = Label::new(None);
                label.set_attributes(&attr_list);
                label.set_label(&emoji);
                label.set_selectable(true);
                label.show();
                grid.attach(&label, column,row as i32,1, 1);
                index += 1;
            }
        }
        row += 1;
    }
}

pub fn build_window(application: &gtk::Application) {
    let emojis : EmojiBase = crate::app::load_emojibase();
    let glade_src = include_str!("emoji_window.glade");
    let builder = Builder::new_from_string(glade_src);

    let window: ApplicationWindow = builder.get_object("main").expect("Couldn't get main window object");
    let search_button: Button = builder.get_object("search-btn").expect("Couldn't get search-button");
    let search_box : Entry = builder.get_object("search-box").expect("Couldn't get search-box");
    let result_grid : Grid = builder.get_object("results-grid").expect("Couldn't get results-grid");
    window.set_application(application);
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });
    search_button.connect_clicked(move |_| {
        let search_phrase = search_box.get_text().expect("Couldn't get text from search-box");
        let search_results = find_emojis(&emojis, search_phrase);
        println!("{:?}", search_results);
        generate_emoji_view(search_results, &result_grid);
    });

    window.show_all();
}