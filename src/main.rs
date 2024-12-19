use raylib::prelude::*;
use std::fs;
use serde_json::Value;

mod special_functions;
//use special_functions::{Ξ, ξ, to_tile_coords};
use special_functions::*;
mod player;
use player::*;


// Constants
pub const TILE_WIDTH:i32 = 32;
pub const TILE_HEIGHT:i32 = 32;

// World constants
pub const G:f32 = 9.816;

fn main() -> Result<(), std::io::Error> {
    // Initializing Raylib
    let (mut rl, thread) = init() // raylib::init()
        .size(1280, 720)
        .title("Isometric Tilemap Renderer")
        .build();
    rl.set_target_fps(60);
    let zoom = 2.;

    // Parse Tile map

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
    let inverted_layers:Vec<(usize, &Vec<Vec<usize>>)> = world_layers.iter().enumerate().rev().collect();

    // World drawing position
    const OFFSET:Vector2 = Vector2::new(600., 200.);
    let mut offset:Vector2 = OFFSET;

    // Dragging logic
    let mut is_dragging:bool = false;
    let (mut drag_start, mut drag_end):(Vector3, Vector3) = (Vector3::zero(), Vector3::zero());
    let mut drag_vector:Vector3;

    // Player
    let mut p1 = Player::new("Player1");

    // Game screen
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        // Mouse
        let mouse_position = rl.get_mouse_position();
            //println!("{:?}", to_tile_coords(mouse_position, zoom, offset, &inverted_layers));
        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging = true;
            drag_start = to_tile_coords(mouse_position.into(), zoom, offset.into(), &inverted_layers);
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging = false;
            drag_end = to_tile_coords(mouse_position.into(), zoom, offset.into(), &inverted_layers);
            drag_vector = drag_start - drag_end;
            println!("drag vector: {:?}", &drag_vector);
        }

        // Keyboard
        // World move
        if rl.is_key_down(KeyboardKey::KEY_LEFT) {
            offset.x -= 1.;
        } else if rl.is_key_down(KeyboardKey::KEY_RIGHT) {
            offset.x += 1.;
        } else if rl.is_key_down(KeyboardKey::KEY_UP) {
            offset.y -= 1.;
        } else if rl.is_key_down(KeyboardKey::KEY_DOWN) {
            offset.y += 1.;
        } else if rl.is_key_down(KeyboardKey::KEY_ENTER) {
            offset = OFFSET;
        }

        // Player move
        if rl.is_key_pressed(KeyboardKey::KEY_W) { p1.velocity += MOVE_UP;
        } else if rl.is_key_released(KeyboardKey::KEY_W) { p1.velocity -= MOVE_UP; } // UP
        if rl.is_key_pressed(KeyboardKey::KEY_S) { p1.velocity += MOVE_DOWN;
        } else if rl.is_key_released(KeyboardKey::KEY_S) { p1.velocity -= MOVE_DOWN; } // DOWN
        if rl.is_key_pressed(KeyboardKey::KEY_D) { p1.velocity += MOVE_RIGHT;
        } else if rl.is_key_released(KeyboardKey::KEY_D) { p1.velocity -= MOVE_RIGHT; } // RIGHT
        if rl.is_key_pressed(KeyboardKey::KEY_A) { p1.velocity += MOVE_LEFT;
        } else if rl.is_key_released(KeyboardKey::KEY_A) { p1.velocity -= MOVE_LEFT; } // LEFT

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
                    let mut screen_coords = Ξ(world_coords.into(), zoom, offset.into());
                    screen_coords.x += TILE_WIDTH as f32/2.;
                    screen_coords.y -= TILE_HEIGHT as f32/2.;
                    let source_rect = texture_lookup[tile - 1];
                    let dest_rect = Rectangle::new(
                        screen_coords.x,
                        screen_coords.y,
                        source_rect.width * zoom,
                        source_rect.height * zoom,
                    );
                    d.draw_texture_pro(&image_texture, source_rect, dest_rect, Vector2::zero(), 0., Color::WHITE);
                }
            }
        }

        // Draw player
        p1.update(dt);
        p1.draw(&mut d, zoom, offset);

        // Draw drag vector
        if is_dragging {
            d.draw_line_ex(
                Ξ(drag_start, zoom, offset.into()),
                mouse_position,
                2.0,
                Color::SKYBLUE,
            );
        }

    }
    Ok(())
}
