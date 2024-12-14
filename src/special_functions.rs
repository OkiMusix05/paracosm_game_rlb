use raylib::ffi::{Vector2, Vector3};

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
pub fn Ξ(r:Vector3, zoom: f32, offset:Vector2) -> Vector2 { //R^3->R^2
    let s = ISOMETRIC_SCALE_CONSTANT * zoom;
    let x = (r.x - r.y) * s + offset.x;
    let y = ((r.x + r.y) / 2.0 - r.z) * s + offset.y;
    Vector2 { x, y }
}
pub fn ξ(p:Vector2, z:f32, zoom:f32, offset:Vector2) -> Vector3 { //R^2->R^3
    let s = ISOMETRIC_SCALE_CONSTANT * zoom;
    let x = (p.y - offset.y + z)/ s + (p.x - offset.x)/(2.* s);
    let y = (p.y - offset.y + z)/ s - (p.x - offset.x)/(2.* s);
    Vector3 { x, y, z }
}