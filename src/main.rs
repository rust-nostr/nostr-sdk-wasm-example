mod app;

use app::App;

fn main() {
    // Init logger
    tracing_wasm::set_as_global_default();

    // Start WASM app
    yew::Renderer::<App>::new().render();
}
