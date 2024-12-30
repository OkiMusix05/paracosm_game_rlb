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
pub const G:f32 = 2.*9.816;

/// Determines if the player can be in that spacial position or not
pub fn world_fn(pos:Vector3, world:&Vec<Vec<Vec<usize>>>, world_width:u64, world_height:u64) -> bool {
    println!("{}, {}, {}", pos.x, pos.y, pos.z);
    let (mut x, mut y, z) = (pos.x as i32, pos.y as i32, pos.z.floor() as usize);
    if z < world.len() - 2 {
        if (x < world_width as i32 && x >= 1) && (y < world_height as i32 && y >= 1) {
            if world[z+1][y as usize][x as usize] != 0 || world[z+2][y as usize][x as usize] != 0 {
                return false;
            }
        }
    }
    true
}
pub fn find_highest_z(p:Vector3, world:&Vec<Vec<Vec<usize>>>) -> f32 {
    let mut highest_z:usize = 0;
    if p.x < 1. || p.y < 1. || p.y >= world[0].len() as f32 { return 0.; }
    if p.x >= world[0][p.y as usize].len() as f32 { return 0.; }
    for (k, z) in world.iter().enumerate() {
        if z[p.y as usize][p.x as usize] > highest_z { highest_z = k; } else { continue }
    }
    highest_z as f32
}

/// Projects vectors into outside a square
/// used when the players position is inside a block it shouldn't be to move it out of it,
/// effectively implementing wall collisions
fn xy_block_collision(p:Vector3) -> Vector3 {
    let grid_x = p.x.floor();
    let grid_y = p.y.floor();

    let local_x = p.x - grid_x - 0.5;
    let local_y = p.y - grid_y - 0.5;

    let scale_x = if local_x != 0.0 { 0.5 / local_x.abs() } else { f32::INFINITY };
    let scale_y = if local_y != 0.0 { 0.5 / local_y.abs() } else { f32::INFINITY };
    const SLIGHT_PUSH:f32 = 0.01;
    let scale = scale_x.min(scale_y) + SLIGHT_PUSH;

    let projected_x = local_x * scale;
    let projected_y = local_y * scale;
    Vector3::new(projected_x + grid_x + 0.5, projected_y + grid_y + 0.5, p.z)
}

fn main() -> Result<(), std::io::Error> {
    // Initializing Raylib
    let (mut rl, thread) = init() // raylib::init()
        .size(1280, 720)
        .title("Isometric Tilemap Renderer")
        .build();
    //rl.set_target_fps(60);
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
    let world_json: Value = serde_json::from_str(&fs::read_to_string("assets/Tiled/world2.tmj")?)?;
    let (world_width, world_height) = {
        (world_json.get("width").and_then(|v| v.as_u64()).expect("1"), world_json.get("height").and_then(|v| v.as_u64()).expect("2"))
    };
    let mut world_layers_raw:Vec<Vec<Vec<usize>>> = Vec::new(); // List of Matrices
    if let Some(layers) = world_json.get("layers").and_then(|v| v.as_array()) {
        for layer in layers.iter() {
            if let Some(data) = layer.get("data").and_then(|v| v.as_array()) {
                let data: Vec<usize> = data.iter()
                    .filter_map(|v| v.as_i64().map(|x| x as usize))
                    .collect();
                let matrix: Vec<Vec<usize>> = data.chunks(world_width as usize)
                    .map(|chunk| chunk.to_vec())
                    .collect();
                world_layers_raw.push(matrix);
            }
        }
    }
    let inverted_layers:Vec<(usize, &Vec<Vec<usize>>)> = world_layers_raw.iter().enumerate().rev().collect();
    let world:Vec<Vec<Vec<usize>>> = {
        let mut world_:Vec<Vec<Vec<usize>>> = vec![];
        for (k, z) in world_layers_raw.iter().enumerate() {
            if k==0 { world_.push(z.clone()); continue; }
            let mut layer: Vec<Vec<usize>> = vec![vec![0; world_width as usize]; world_height as usize];
            for (j, y) in z.iter().enumerate() {
                for (i, x) in y.iter().enumerate() {
                    if i+k < world_width as usize && j+k < world_height as usize {
                        layer[j+k][i+k] = z[j][i]; // later check for z[j][i]'s number. 1 would represent a solid block and so on
                    }
                }
            }
            world_.push(layer);
        }
        world_.push(vec![vec![0; world_width as usize]; world_height as usize]); // world top
        world_.push(vec![vec![0; world_width as usize]; world_height as usize]);
        world_
    };

    // World drawing position
    const OFFSET:Vector2 = Vector2::new(800., 150.);
    let mut offset:Vector2 = OFFSET;

    // Dragging logic
    let mut is_clicking:bool = false;
    let mut jump_timer:f32 = 0.;
    let mut jump_number:u8 = 0;
    /*let mut is_dragging:bool = false;
    let (mut drag_start, mut drag_end):(Vector3, Vector3) = (Vector3::zero(), Vector3::zero());
    let mut drag_vector:Vector3;*/

    // Player
    let mut p1 = Player::new("Player1");

    // Game screen
    while !rl.window_should_close() {
        let dt = rl.get_frame_time();
        let fps = rl.get_fps();
        // Mouse
        let mouse_position = rl.get_mouse_position();
        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) { is_clicking = true; } else { is_clicking = false; }
        //println!("{:?}", to_tile_coords(mouse_position, zoom, offset, &inverted_layers));
        /*if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging = true;
            drag_start = to_tile_coords(mouse_position.into(), zoom, offset.into(), &inverted_layers);
        }
        if rl.is_mouse_button_released(MouseButton::MOUSE_BUTTON_LEFT) {
            is_dragging = false;
            drag_end = to_tile_coords(mouse_position.into(), zoom, offset.into(), &inverted_layers);
            drag_vector = drag_start - drag_end;
            println!("drag vector: {:?}", &drag_vector);
        }*/

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
        if rl.is_key_pressed(KeyboardKey::KEY_W) {
            p1.velocity += MOVE_UP;
        } else if rl.is_key_released(KeyboardKey::KEY_W) { p1.velocity -= MOVE_UP; } // UP
        if rl.is_key_pressed(KeyboardKey::KEY_S) {
            p1.velocity += MOVE_DOWN;
        } else if rl.is_key_released(KeyboardKey::KEY_S) { p1.velocity -= MOVE_DOWN; } // DOWN
        if rl.is_key_pressed(KeyboardKey::KEY_D) {
            p1.velocity += MOVE_RIGHT;
        } else if rl.is_key_released(KeyboardKey::KEY_D) { p1.velocity -= MOVE_RIGHT; } // RIGHT
        if rl.is_key_pressed(KeyboardKey::KEY_A) {
            p1.velocity += MOVE_LEFT;
        } else if rl.is_key_released(KeyboardKey::KEY_A) { p1.velocity -= MOVE_LEFT; } // LEFT
        jump_timer += dt;
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            jump_number += 1;
            if jump_timer < 0.3 && jump_number < 2 { // Double jump window
                p1.velocity.z += 7.;
            }
        }
        // add dash mechanic with shift and a timer
        println!("v = {}", p1.velocity.z);
        //println!("jt = {}, {}", jump_timer, jump_number);
        //println!("{:?}", world_fn(p1.position, &world, world_width, world_height));

        // Prevent player from leaving the boundaries of the world
        if p1.position.x > world_width as f32 - 1. {
            p1.position.x = world_width as f32 - 1.05;
        } else if p1.position.x < 1. {
            p1.position.x = 1.;
        }
        if p1.position.y > world_height as f32 - 1.{
            p1.position.y = world_height as f32 - 1.05;
        } else if p1.position.y < 1. {
            p1.position.y = 1.;
        }

        // Give the player gravity, jump, and ground detection
        let highest_z = find_highest_z(p1.position, &world);
        let height_diff = p1.position.z - highest_z;
        if height_diff < 0. && height_diff > -0.05 && p1.velocity.z < 0.0 {
            p1.position.z = highest_z;
            p1.velocity.z = 0.;
        }
        if p1.position.z == highest_z { jump_timer = 0.; jump_number = 0; }

        // world collisions
        match world_fn(p1.position, &world, world_width, world_height) {
            true => {}
            false => {
                p1.position = xy_block_collision(p1.position);
            }
        }

        // Drawing
        let mut d = rl.begin_drawing(&thread);

        // Background
        d.clear_background(Color::BLACK);

        // World Render
        let (p1_x, p1_y) = get_int_xy_position(p1.position);
        let p1_z = p1.position.z as usize;
        for (z, layer) in world_layers_raw.iter().enumerate() {
            for (y, m_row) in layer.iter().enumerate() {
                for (x, tile) in m_row.iter().enumerate() {
                    if *tile == 0 {continue}
                    /*if layer[p1_y+1][p1_x] != 0 || layer[p1_y][p1_x+1] != 0 || layer[p1_y+1][p1_x+1] != 0 {
                        //p1.draw(&mut d, zoom, offset);
                    }*/
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
                    let color = if p1_x == x + z && p1_y == y + z{Color::DIMGRAY} else {Color::WHITE};
                    d.draw_texture_pro(&image_texture, source_rect, dest_rect, Vector2::zero(), 0., color);
                }
            }
        }

        // Draw player
        p1.update(&world, dt);
        //let shadow_pos = Ξ(Vector3::new(p1.position.x, p1.position.y, 0.), zoom, offset.into());
        //d.draw_circle(shadow_pos.x as i32 + TILE_WIDTH + TILE_WIDTH/2, shadow_pos.y as i32 - TILE_HEIGHT/2, 16., Color::GRAY);
        p1.draw(&mut d, zoom, offset);

        // Draw drag vector
        /*if is_dragging {
            d.draw_line_ex(
                Ξ(drag_start, zoom, offset.into()),
                mouse_position,
                2.0,
                Color::SKYBLUE,
            );
        }*/
        if is_clicking {
            d.draw_line_ex(
                Ξ(Vector3::new(p1.position.x, p1.position.y-1., p1.position.z), zoom, offset.into())+Vector2::new(0.5*TILE_WIDTH as f32,-1.0*TILE_HEIGHT as f32),
                mouse_position,
                2.0,
                Color::SKYBLUE,
            );
        }
        d.draw_text(&format!("FPS: {}", fps), 10, 10, 20, Color::WHITE);

    }
    Ok(())
}
