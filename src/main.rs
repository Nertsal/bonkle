mod assets;
mod collection;
mod config;
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
    #[clap(long, default_value = "config.ron")]
    config: PathBuf,
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
            let config = config::Config::load(&args.config).await.unwrap();
            game::Game::new(&geng, &Rc::new(assets), config)
        }
    };

    geng.run_loading(future)
}
