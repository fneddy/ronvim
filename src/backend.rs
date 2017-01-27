use qml::*;

#[derive(Default)]
pub struct Backend;

impl QBackend {
    pub fn handle_input(&self, txt: String, key: i32, modifiers: i32, scancode: i32) ->  Option<&QVariant> {
        //println!("input: {:#?} {:#X} {:#X} {:#X}",txt, key, modifiers, scancode);
        let s: String = self.get_text().into();
        println!("{:?}",s);
        None
    }
    pub fn open(&self, path: String) ->  Option<&QVariant> {
        println!("open file:{:?}",path);
        None
    }
    pub fn save(&mut self, path: String) ->  Option<&QVariant> {
        self.set_filename(path);
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
            name: String; read: get_name, write: set_name, notify: name_changed;
            text: String; read: get_text, write: set_text, notify: text_changed;
            filename: String; read: get_filename, write: set_filename, notify: filename_changed;
    }
);

Q_REGISTERABLE_QML!(QBackend: Backend as RsBackend 1=>0, from BackendModule);
