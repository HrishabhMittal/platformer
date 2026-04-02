use crate::network::Vec2;

pub fn is_target_hit(shooter_pos: Vec2, mouse_pos: Vec2, target_rect: (f32, f32, f32, f32)) -> bool {
    let (rx, ry, rw, rh) = target_rect;
    
    let a = mouse_pos.y - shooter_pos.y;
    let b = shooter_pos.x - mouse_pos.x;
    let c = (mouse_pos.x * shooter_pos.y) - (shooter_pos.x * mouse_pos.y);

    let corners = [
        (rx, ry),               
        (rx + rw, ry),          
        (rx, ry + rh),          
        (rx + rw, ry + rh),     
    ];

    let mut has_positive = false;
    let mut has_negative = false;

    for (cx, cy) in corners.iter() {
        let eval = a * cx + b * cy + c;
        if eval > 0.0 { has_positive = true; } 
        else if eval < 0.0 { has_negative = true; }
    }

    if !(has_positive && has_negative) {
        return false;
    }

    let shot_dir_x = mouse_pos.x - shooter_pos.x;
    let shot_dir_y = mouse_pos.y - shooter_pos.y;
    let to_target_x = (rx + rw / 2.0) - shooter_pos.x;
    let to_target_y = (ry + rh / 2.0) - shooter_pos.y;

    let dot_product = (shot_dir_x * to_target_x) + (shot_dir_y * to_target_y);
    dot_product > 0.0
}
