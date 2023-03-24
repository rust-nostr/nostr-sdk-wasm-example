mod app;

use app::App;

fn main() {
    // Init logger
    wasm_logger::init(wasm_logger::Config::default());

    // Start WASM app
    yew::Renderer::<App>::new().render();
}
