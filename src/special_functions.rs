use raylib::math::{Vector2, Vector3};

/*pub fn Ξ(x:f32, y:f32, z:f32, s: f32, offset: (f32, f32)) -> (f32, f32) { //R^3->R^2
    let u = (x - y) * s + offset.0;
    let v = ((x + y) / 2.0 - z) * s + offset.1;
    (u, v)
}*/
/*pub fn ξ(u:f32, v:f32, z:f32, s:f32, offset: (f32, f32)) -> (f32, f32, f32) { //R^2->R^3
    let x = (v - offset.1 + z)/s + (u - offset.0)/(2.*s);
    let y = (v - offset.1 + z)/s - (u - offset.0)/(2.*s);
    (x, y, z)
}*/
pub const ISOMETRIC_SCALE_CONSTANT:f32 = 16.;
#[allow(non_snake_case)]
/// Transformation to Isometric screen space
pub fn Ξ(r:Vector3, zoom: f32, offset:Vector2) -> Vector2 { //R^3->R^2
    let s = ISOMETRIC_SCALE_CONSTANT * zoom;
    let x = (r.x - r.y) * s + offset.x;
    let y = ((r.x + r.y) / 2.0 - r.z) * s + offset.y;
    Vector2 { x, y }
}
/// Inverse Transform: From screen to world space
pub fn ξ(p:Vector2, z:f32, zoom:f32, offset:Vector2) -> Vector3 { //R^2->R^3
    let s = ISOMETRIC_SCALE_CONSTANT * zoom;
    let x = (p.y - offset.y + z)/ s + (p.x - offset.x)/(2.* s);
    let y = (p.y - offset.y + z)/ s - (p.x - offset.x)/(2.* s);
    Vector3 { x, y, z }
}

/// Transformation to world coordinates
pub fn to_tile_coords(p:Vector2, zoom:f32, offset:Vector2, inverse_layers:&Vec<(usize, &Vec<Vec<usize>>)>) -> Vector3 {
    let world_position = ξ(p.into(), 0., zoom, offset.into());
    let index_pos = Vector3 {
        x: ((world_position.x - 0.5) as i32) as f32, // The 0.5's are to offset the block center
        y: ((world_position.y + 0.5) as i32) as f32, // Cause +y is down and +x is right
        z: 0.0,
    };
    let mut get_z = 0;
    for (k, layer) in inverse_layers {
        if index_pos.y as usize >= layer.len() || index_pos.x as usize >= layer[index_pos.y as usize].len() { get_z =0; break;}
        if layer[index_pos.y as usize][index_pos.x as usize] != 0 { get_z = *k; break} // layer(y,x)
        else {continue}
    }
    Vector3 {
        x: world_position.x + get_z as f32,
        y: world_position.y + get_z as f32,
        z: get_z as f32,
    }
}