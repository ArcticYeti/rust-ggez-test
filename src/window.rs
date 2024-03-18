use ggez::winit::{dpi::LogicalPosition, window::Window};

pub fn center_window_to_screen(win: &Window) {
    match win.current_monitor() {
        Some(display) => {
            let display_size = display.size();
            let window_size = win.outer_size();
        
            let center_x = match display_size.width.checked_sub(window_size.width) {
                Some(diff) => diff / 2,
                None => 0, // Fallback if subtraction overflows
            };
            let center_y = match display_size.height.checked_sub(window_size.height) {
                Some(diff) => diff / 2,
                None => 0, // Fallback if subtraction overflows
            };

            win.set_outer_position(LogicalPosition::new(center_x, center_y));
        }
        None => {
            return;
        }
    }
}
