use bevy::prelude::*;
use std::collections::HashSet;

const GRID_WIDTH: usize = 2000;
const GRID_HEIGHT: usize = 2000;

#[derive(Hash, Eq, PartialEq)]
enum Tile {
    Grass,
    Water,
    Sand,
}

#[derive(Component)]
struct Cell {
    x_coord: usize,
    y_coord: usize,
    possibilites: HashSet<Tile>,
    collapsed: bool,
}

impl Cell {
    fn new(x_coord: usize, y_coord: usize) -> Self {
        let possibilities: HashSet<Tile> = vec![Tile::Grass, Tile::Water, Tile::Sand]
            .into_iter()
            .collect();

        Self {
            x_coord,
            y_coord,
            possibilites: possibilities,
            collapsed: false,
        }
    }
}

#[derive(Resource)]
struct Grid {
    width: usize,
    height: usize,
    rule: Vec<(Tile, Tile)>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            rule: vec![
                (Tile::Grass, Tile::Sand),
                (Tile::Grass, Tile::Water),
                (Tile::Water, Tile::Water),
            ],
        }
    }
}

fn initialize(mut commands: Commands, grid: Option<Res<Grid>>) {
    if let Some(grid) = &grid {
        for x in 0..grid.width {
            for y in 0..grid.height {
                commands.spawn(Cell::new(x, y));
            }
        }
    }
}

fn check_cells(query: Query<&Cell>) {
                       for cell in query {
        println!("[{}, {}]", cell.x_coord, cell.y_coord);
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Grid>()
        .add_systems(Startup, (initialize, check_cells).chain())
        .run();
}
