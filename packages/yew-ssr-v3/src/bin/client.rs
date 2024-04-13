use yew_ssr_v3::App;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_logger::init(wasm_logger::Config::new(log::Level::Trace));

    println!("Do I hydrate?");
    
    yew::Renderer::<App>::new().hydrate();
}

