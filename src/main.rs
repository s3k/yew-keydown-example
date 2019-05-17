use wservice::{Model, Msg};
use yew::App;

// Init app with Msg::ListenKeydown
fn main() {
    yew::initialize();

    let app = App::<Model>::new();
    app.mount_to_body().send_message(Msg::ListenKeydown);

    yew::run_loop();
}
