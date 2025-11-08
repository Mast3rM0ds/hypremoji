use std::{cell::RefCell, rc::Rc};

use gtk::{
    gdk::Key, prelude::{GtkWindowExt, WidgetExt, EditableExt}, ApplicationWindow, Entry, EventControllerKey 
};

pub fn setup_keyboard_controller(
    window: &ApplicationWindow,
    search_input_rc: Rc<RefCell<Entry>>,
) -> EventControllerKey {
    let key_controller = EventControllerKey::new();
    let window_clone: ApplicationWindow = window.clone();

    key_controller.connect_key_pressed(move |_controller, key, _keycode, _state| {
        if key == Key::Escape {
            window_clone.close();
            return gtk::glib::Propagation::Stop;
        }

        let special_keys = [
            Key::Return,
            Key::KP_Enter,
            Key::Tab,
            Key::ISO_Left_Tab,
            Key::Control_L,
            Key::Control_R,
            Key::Shift_L,
            Key::Shift_R,
            Key::Alt_L,
            Key::Alt_R,
            Key::Super_L,
            Key::Super_R,
        ];

        if special_keys.contains(&key) {
            return gtk::glib::Propagation::Proceed;
        }
        
        if let Some(unicode) = key.to_unicode() {
            let search_input = search_input_rc.borrow();
            if !search_input.has_focus() {
                search_input.grab_focus();
                
                let current_text = search_input.text();
                let cursor_pos = search_input.position();
                let mut new_text = current_text.to_string();
                new_text.insert(cursor_pos as usize, unicode);
                search_input.set_text(&new_text);
                search_input.set_position(cursor_pos + 1);
                
                return gtk::glib::Propagation::Stop;
            }
        }
        
        gtk::glib::Propagation::Proceed
    });

    key_controller
}