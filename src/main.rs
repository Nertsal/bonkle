mod assets;
mod collection;
mod game;
mod model;
mod render;
mod util;

use geng::prelude::*;

const FPS: f64 = 60.0;

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Bonkle".to_string(),
        fixed_delta_time: 1.0 / FPS,
        fullscreen: true,
        ..default()
    });

    let future = {
        let geng = geng.clone();
        async move {
            let assets: assets::Assets =
                geng::Load::load(geng.asset_manager(), &run_dir().join("assets"))
                    .await
                    .expect("failed to load assets");
            game::Game::new(&geng, &Rc::new(assets))
        }
    };

    geng.run_loading(future)
}
