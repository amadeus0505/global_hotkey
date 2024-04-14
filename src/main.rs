mod winapi_hook;
mod global_hotkey_;
mod find_keycode;

fn main() {
    global_hotkey_::start();
}