use raylib::prelude::*;
use std::fs;
use serde_json::Value;

mod special_functions;
use special_functions::{Ξ, ξ};

fn main() -> Result<(), std::io::Error> {
    // Initializing Raylib
    let (mut rl, thread) = init() // raylib::init()
        .size(1280, 720)
        .title("Isometric Tilemap Renderer")
        .build();
    rl.set_target_fps(60);
    let zoom_factor = 2.;

    // Parse Tile map
    const TILE_WIDTH:i32 = 32;
    const TILE_HEIGHT:i32 = 32;

    let image_texture = rl.load_texture(&thread, "assets/Tiled/ground_tiles.png").unwrap();
    let mut texture_lookup:Vec<Rectangle> = {
        let mut lookup:Vec<Rectangle> = Vec::new();
        let (tileset_columns, tileset_rows) = (image_texture.width() / TILE_WIDTH, image_texture.height() / TILE_HEIGHT);
        let tile_count = tileset_columns * tileset_rows;
        for tile_id in 0..tile_count {
            let tile_x = (tile_id % tileset_columns) * TILE_WIDTH;
            let tile_y = (tile_id / tileset_columns) * TILE_HEIGHT;

            let source_rect = Rectangle::new(tile_x as f32, tile_y as f32, TILE_WIDTH as f32, TILE_HEIGHT as f32);

            lookup.push(source_rect);
        }
        lookup
    };

    // World Tile map arrangements
    let world_json: Value = serde_json::from_str(&fs::read_to_string("assets/Tiled/world1.tmj")?)?;
    let (world_width, _) = {
        (world_json.get("width").and_then(|v| v.as_u64()).expect("1"), world_json.get("height").and_then(|v| v.as_u64()).expect("2"))
    }; // _ is world_height
    let mut world_layers:Vec<Vec<Vec<usize>>> = Vec::new(); // List of Matrices
    if let Some(layers) = world_json.get("layers").and_then(|v| v.as_array()) {
        for layer in layers.iter() {
            if let Some(data) = layer.get("data").and_then(|v| v.as_array()) {
                let data: Vec<usize> = data.iter()
                    .filter_map(|v| v.as_i64().map(|x| x as usize))
                    .collect();
                let matrix: Vec<Vec<usize>> = data.chunks(world_width as usize)
                    .map(|chunk| chunk.to_vec())
                    .collect();
                world_layers.push(matrix);
            }
        }
    }
    //world_layers = vec![world_layers[0].clone()];

    const OFFSET:Vector2 = Vector2::new(600., 200.);

    while !rl.window_should_close() {
        // Mouse
        let mouse_position = rl.get_mouse_position();
        let mouse_world_position = ξ(mouse_position.into(), 0., zoom_factor, OFFSET.into());
        let index_pos = Vector3 {
            x: ((mouse_world_position.x - 0.5) as i32) as f32,
            y: ((mouse_world_position.y + 0.5) as i32) as f32, // Cause +y is down and +x is right
            z: 0.0,
        };
        let mut new_z = 0;
        let rev_arr:Vec<(usize, &Vec<Vec<usize>>)> = world_layers.iter().enumerate().rev().collect();
        for (k, layer) in rev_arr.clone() {
            if index_pos.y as usize >= layer.len() || index_pos.x as usize >= layer[index_pos.y as usize].len() {new_z=0; break;}
            if layer[index_pos.y as usize][index_pos.x as usize] != 0 {new_z = k; break} // layer(y,x)
            else {continue}
        }
        let mouse_is_in = Vector3 {
            x: index_pos.x,
            y: index_pos.y,
            z: new_z as f32,
        };
        println!("{:?}", mouse_is_in);

        // Drawing
        let mut d = rl.begin_drawing(&thread);

        // Background
        d.clear_background(Color::BLACK);

        // World Render
        for layer in &world_layers {
            for (y, m_row) in layer.iter().enumerate() {
                for (x, tile) in m_row.iter().enumerate() {
                    if *tile == 0 {continue}
                    // !! Add check to see if there's a tile in a layer above it, then don't render it
                    let world_coords = Vector3::new(x as f32, y as f32, 0.);
                    let screen_coords = Ξ(world_coords.into(), zoom_factor, OFFSET.into());
                    let source_rect = texture_lookup[tile - 1];
                    let dest_rect = Rectangle::new(
                        screen_coords.x,
                        screen_coords.y,
                        source_rect.width * zoom_factor,
                        source_rect.height * zoom_factor,
                    );
                    d.draw_texture_pro(&image_texture, source_rect, dest_rect, Vector2::zero(), 0., Color::WHITE);
                }
            }
        }
    }
    Ok(())
}
