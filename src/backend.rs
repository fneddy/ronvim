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
        let mut n = Neovim::new(session);
        n.input("i");
        Backend {
            nvim: n
        }
    }
}

impl QBackend {
    pub fn sync_buffers(&mut self) {
        let b = self.nvim.get_current_buffer().unwrap();
        let l = b.line_count(&mut self.nvim).unwrap();
        let s = b.get_line_slice(&mut self.nvim, 0, l,true, true).unwrap().join("\n");

        let p = self.nvim.call_function("getcurpos",Vec::new());
        println!("pos: {:?}",p);

        self.set_text(s);
        self.text_changed();
    }

    pub fn handle_input(&mut self, txt: String, key: i32, modifiers: i32, scancode: i32) ->  Option<&QVariant> {
        println!("> {:#?} {:#?}",txt,txt.chars());
        if txt.as_bytes().len() == 0 {
           //return Some(false.into())
           return None
        }
        let b = txt.as_bytes()[0] ;
        if b < 0x20  && ! (b == 0x0D || b == 0x08) {
           //return Some(QVariant::from(false))
           return None
        }

        self.nvim.input(&txt);

        self.sync_buffers();
        //return Some(QVariant::from(true))
        None
    }

    pub fn open(&mut self, path: String) ->  Option<&QVariant> {
        let f = Url::parse(&path).unwrap();
        let p = Path::new(f.path());
        self.nvim.command(&format!(":e {}", p.display()));
        self.sync_buffers();
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
            cursor_position: i32; read: get_position, write: set_position, notify: position_changed;
            selection_start: String; read: get_sel_start, write: set_sel_start, notify: sel_start_changed;
            selection_end: String; read: get_sel_end, write: set_sel_end, notify: sel_end_changed;
    }
);

Q_REGISTERABLE_QML!(QBackend: Backend as RsBackend 1=>0, from BackendModule);
