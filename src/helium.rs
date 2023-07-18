/*
	Helium v1

	text editor applet for Midnight
	based on `tui-textarea`
	fully command based
*/

// Designed as a Midnight command
//  - Open a new Helium panel with no content
pub fn blank_editor(i: &mut Interface) {
	i.new_centered_panel(
		Panel::new()
	);
}

// Designed as a Midnight command
//  - Open a new Helium panel with an exisiting file
pub fn open_editor(i: &mut Interface) {
	let path = i.popup_prompt(String::new("Enter files path: "));
	// EXTRA: Use `crate::pandocs` for DocumentX integration
	// Get the content
	// Create new panel
	// Send file content
}

// Designed as a Midnight command
//  - Save current Helium panel
pub fn save_editor(i: &mut Interface) {
	if (i.current_panel_applet() == String::new("helium_v1")) {
		// Get the content
		// Get the file path
		// Save on the filesystem
	} else {
		i.popup_text(String::new("Impossible to save this Panel's content beacuse it's not based on Helium v1."));
	}
}

// Yes, Prompt is implemented inside the `helium.rs` source file
// This is beacuse Prompt itself uses Helium.
impl Interface {
	pub fn _prompt(&mut self, text: String) -> String {
		let original_bottom = self.bottom_panel;

		
	}
}