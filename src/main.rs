use bevy::prelude::*;

// Globals
const WIDTH: usize = 32;
const HEIGHT: usize = 32;

enum Block {
    Mountain,
    River,
    Forest,
    City,
    Crop,
}

#[derive(Resource)]
struct Grid {
    width: usize,
    height: usize,
}

#[derive(Component)]
struct Cell {
    position: [usize; 2],
    block: Block,
}

#[derive(Component)]
struct Rule {

}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: WIDTH,
            height: HEIGHT,
        }
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}