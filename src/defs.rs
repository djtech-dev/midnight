use std::collections::HashMap;
use ratatui::widgets::Widget;

pub struct Panel {
    widget: Box<dyn Widget>,

    height: f32, // Percentual of terminal's height
    width: f32,  // Percentual of terminal's widht 
}

//pub struct Interface<F: FnOnce() -> String> {
pub struct Interface {
    // Tiling Panel Managment
    top_panel: Option<Panel>,
    bottom_panel: Option<Panel>,
    panels: Vec<Panel>,

    // Equivalent to Emacs's commands.
    // Use `M+x` in order to launch the Command Prompt
    
    commands: HashMap<String, Box<dyn FnMut(&mut Interface) -> ()>>,
}

impl Interface {
    pub fn new() -> Interface {
        Interface {
            top_panel: None,
            bottom_panel: None,
            panels: Vec::new(),
            commands: HashMap::new(),
        }
    }

    pub fn render() {
        // 1. Render Top Panel
        // 2. Render All Center Panels
        // 3. Render Bottom Panel
    }   
}
