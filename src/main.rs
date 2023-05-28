mod assets;
mod game;

use geng::prelude::*;

const FIXED_DELTA_TIME: f32 = 1.0 / 60.0;

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Bonkle".to_string(),
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
