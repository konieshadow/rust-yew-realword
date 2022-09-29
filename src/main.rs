use console_error_panic_hook::set_once as set_panic_hook;
use yew_web_app::app::App;

fn main() {
    set_panic_hook();

    yew::start_app::<App>();
}
