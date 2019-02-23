use super::*;


pub struct DisplayEventHandler {
    pub config: Config
}


impl DisplayEventHandler{


    /*
     * process a collection of events, and apply them changes to a manager
     * returns true if changes need to be rendered
     */
    pub fn process(&self, manager: &mut Manager, event: DisplayEvent) -> bool {
        //println!("EVENT: {:?}", event);
        //println!("state: {:?}", manager);
        let update_needed = match event {
            DisplayEvent::ScreenCreate(s) => screen_create_handler::process(manager, s ),
            DisplayEvent::WindowCreate(w) => window_handler::created(manager, w ),
            DisplayEvent::FocusedWindow(handle) => focus_handler::focus_window_by_handle( manager, &handle ),
            DisplayEvent::WindowDestroy(handle) => window_handler::destroyed(manager, &handle ),
            DisplayEvent::KeyCombo(mod_mask, xkeysym) => {
                //look through the config and build a command if its defined in the config
                let build = CommandBuilder::new(&self.config);
                let command = build.from_xkeyevent( mod_mask, xkeysym );
                if let Some((cmd, val)) = command {
                    return command_handler::process(manager, cmd, val);
                }
                false
            },
        };

        if update_needed {
            self.update_windows(manager);
        }

        update_needed
    }


    /*
     * step over all the windows for each workspace and updates all the things
     * based on the new state of the WM
     */
    fn update_windows(&self, manager: &mut Manager) {
        let all_windows = &mut manager.windows;
        let all: Vec<&mut Window> = all_windows.iter_mut().map(|w| w).collect();
        for w in all {
            w.visable = w.tags.is_empty();
        } // if not tagged force it to display
        for ws in &mut manager.workspaces {
            let windows: Vec<&mut Window> = all_windows.iter_mut().map(|w| w).collect();
            ws.update_windows(windows);
        }
    }


}


