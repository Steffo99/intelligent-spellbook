mod app;
mod macros;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
