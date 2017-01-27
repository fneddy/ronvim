use qml::*;
use {Backend,QBackend};

pub struct Frontend{
    backends: Vec<Box<QBackend>>,
}

#[derive(Debug, PartialEq)]
enum Modifier {
    None = 0,
    Shift= 0x2000000,
    Ctrl = 0x4000000,
    Alt = 0x8000000,
    Meta = 0x10000000,
    KeyPad = 0x20000000,
}

impl Frontend {
    pub fn new() -> Frontend {
        Frontend { backends: Vec::new() }
    }
    
    fn run_command(&self, cmd: String) ->  Option<&QVariant> {
        println!("run: {}",cmd);
        None
    }
}


Q_OBJECT!(
    pub Frontend as QFrontend {
        signals:
        slots:
            fn run_command(cmd: String);
        properties:
    }
);

impl QFrontend {
    pub fn create() -> Box<QFrontend> {
        QFrontend::new(Frontend::new())
    }
}
