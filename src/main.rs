mod assets;
mod game;
mod model;
mod render;
mod util;

use std::path::PathBuf;

use clap::Parser;
use geng::prelude::*;

const FPS: f64 = 60.0;

#[derive(Parser)]
struct Args {
    #[clap(long, default_value = "assets/config.ron")]
    config: PathBuf,
    #[clap(long, default_value = "assets/themes/default.toml")]
    theme: PathBuf,
    #[clap(flatten)]
    geng: geng::CliArgs,
}

fn main() {
    logger::init();
    geng::setup_panic_handler();

    let args: Args = clap::Parser::parse();

    let geng = Geng::new_with(geng::ContextOptions {
        title: "Bonkle".to_string(),
        fixed_delta_time: 1.0 / FPS,
        fullscreen: true,
        ..geng::ContextOptions::from_args(&args.geng)
    });

    let future = {
        let geng = geng.clone();
        async move {
            let manager = geng.asset_manager();
            let assets = assets::Assets::load(manager).await.unwrap();
            let config = geng::asset::Load::load(manager, &args.config)
                .await
                .unwrap();
            let theme = geng::asset::Load::load(manager, &args.theme).await.unwrap();
            let entities =
                geng::asset::Load::load(manager, &run_dir().join("assets").join("entities"))
                    .await
                    .unwrap();
            game::Game::new(&geng, &Rc::new(assets), config, theme, entities)
        }
    };

    geng.run_loading(future)
}
