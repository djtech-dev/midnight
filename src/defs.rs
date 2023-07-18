use ratatui::layout::Layout;
use std::collections::HashMap;

pub struct Panel {
    //content: Box<dyn Widget>,
    content: Layout,    

    height: f32, // Percentual of terminal's height
    width: f32,  // Percentual of terminal's widht
}

pub struct Interface {
    // Tiling Panel Managment
    top_panel: Option<Panel>,
    bottom_panel: Option<Panel>,
    panels: Vec<Panel>,

    // Equivalent to Emacs's commands.
    // Use `M+x` in order to launch the Command Prompt
    commands: HashMap<String, Box<dyn FnMut(&mut Interface) -> ()>>,
}

impl Panel {
    pub fn new(content: Layout, height: f32, width: f32) -> Panel {
        Panel {
            content: content,
            height: height,
            width: width,
        }
    }

    pub fn update_dimension(&mut self) {

    }

    pub fn set_height(&mut self, height: f32) {
        self.height = height;
        self.update_dimension();
    }
    pub fn set_width(&mut self, width: f32) {
        self.width = width;
        self.update_dimension();
    }
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
