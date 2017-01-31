use qml::*;
use neovim_lib::session::Session;
use neovim_lib::neovim::Neovim;
use neovim_lib::neovim_api::{NeovimApi, Buffer};

use url::Url;
use std::path::Path;

pub struct Backend{
    nvim: Neovim,

}
impl Default for Backend {
    fn default() -> Backend {
        let mut session = Session::new_child().unwrap();
        session.start_event_loop();

        Backend {
            nvim: Neovim::new(session),
        }
    }
}

impl QBackend {
    pub fn p(&mut self) {
        let b = self.nvim.get_current_buffer().unwrap();
        let s = b.get_line_slice(&mut self.nvim, 0, 30,true, true);
        println!("{:#?}",s);
    }

    pub fn handle_input(&mut self, txt: String, key: i32, modifiers: i32, scancode: i32) ->  Option<&QVariant> {

        println!("> {:#?}",txt);

        self.nvim.input(&txt);

        self.p();
        None
    }

    pub fn open(&mut self, path: String) ->  Option<&QVariant> {
        let f = Url::parse(&path).unwrap();
        let p = Path::new(f.path());
        self.nvim.command(&format!(":e {}", p.display()));
        self.p();
        None
    }

    pub fn save(&mut self, path: String) ->  Option<&QVariant> {
        None
    }
}

Q_OBJECT!(
    pub Backend as QBackend {
        signals:
        slots:
            fn handle_input( txt: String, key: i32, modifiers: i32, scancode: i32);
            fn open( path: String);
            fn save( path: String);
        properties:
            text: String; read: get_text, write: set_text, notify: text_changed;
            cursor_position: String; read: get_position, write: set_position, notify: position_changed;
            selection_start: String; read: get_sel_start, write: set_sel_start, notify: sel_start_changed;
            selection_end: String; read: get_sel_end, write: set_sel_end, notify: sel_end_changed;
    }
);

Q_REGISTERABLE_QML!(QBackend: Backend as RsBackend 1=>0, from BackendModule);
