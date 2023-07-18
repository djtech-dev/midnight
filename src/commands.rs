// Collection of various Midnight commands
// Mostly Tiling-related commands

use std::process::exit;

// Quit midnight
pub fn quit_all(i: &mut Interface) {
	exit(0);
}

// Remove panel
pub fn remove_panel(i: &mut Interface) {}
pub fn absolute_resize_height(i: &mut Interface) {}
pub fn relative_resize_height(i: &mut Interface) {}
pub fn absolute_resize_weigth(i: &mut Interface) {}
pub fn relative_resize_weigth(i: &mut Interface) {}
pub fn move_by_left(i: &mut Interface) {}
pub fn move_by_right(i: &mut Interface) {}
pub fn move_by_up(i: &mut Interface) {}
pub fn move_by_down(i: &mut Interface) {}
pub fn vertical_split(i: &mut Interface) {}
pub fn vertical_join(i: &mut Interface) {}
pub fn move_full_left(i: &mut Interface) {}
pub fn move_full_right(i: &mut Interface) {}
pub fn move_to_tabs(i: &mut Interface) {}
pub fn move_from_tabs(i: &mut Interface) {}