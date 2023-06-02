use super::BodyConfig;

use geng::prelude::futures::FutureExt;
use geng::prelude::*;

#[derive(Debug, Clone)]
pub struct EntitiesAssets {
    pub configs: HashMap<String, BodyConfig>,
}

impl geng::Load for EntitiesAssets {
    fn load(manager: &geng::Manager, path: &std::path::Path) -> geng::asset::Future<Self> {
        let _manager = manager.clone();
        let path = path.to_owned();
        async move {
            let list: Vec<String> = file::load_detect(path.join("_list.ron")).await?;
            let mut configs = HashMap::new();
            for name in list {
                let config: BodyConfig =
                    file::load_detect(&path.join(format!("{name}.ron"))).await?;
                configs.insert(name, config);
            }
            Ok(Self { configs })
        }
        .boxed_local()
    }

    const DEFAULT_EXT: Option<&'static str> = None;
}
