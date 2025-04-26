use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct SceneAssets {
    pub asteroid: Handle<Scene>,
    pub spaceship: Handle<Scene>,
    pub missiles: Handle<Scene>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SceneAssets>()
            .add_systems(Startup, load_assets);
    }
}

fn load_assets(mut scene_assets: ResMut<SceneAssets>, asset_server: Res<AssetServer>) {
    *scene_assets = SceneAssets {
        asteroid: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Asteroid.glb")),
        spaceship: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Spaceship.glb")),
        missiles: asset_server.load(GltfAssetLabel::Scene(0).from_asset("Missiles.glb")),
    }
}
