use geng::prelude::*;

#[derive(geng::Load)]
pub struct Assets {
    pub body_hit: geng::Sound,
    pub head_hit: geng::Sound,
    pub death: geng::Sound,
    pub bounce: geng::Sound,
    pub explosion: geng::Sound,
    #[load(postprocess = "looping")]
    pub music: geng::Sound,
    #[load(postprocess = "pixel")]
    pub tutorial: ugli::Texture,
}

fn looping(sfx: &mut geng::Sound) {
    sfx.set_looped(true)
}

fn pixel(texture: &mut ugli::Texture) {
    texture.set_filter(ugli::Filter::Nearest);
}
