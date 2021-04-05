pub struct Minion {
    id: String,
    platform: String,
    arch: String,
    plugins: Vec<String>,
    ram: i32,  // in GB
    disk: i32, // in GB
    has_root: bool,
}

impl Minion {
    // run a command on this minion
    pub fn command(_command: String, _arguments: Vec<String>) {}

    // install a plugin on this minion
    pub fn install(_plugin: String) {}
}

pub struct Rattata {
    // list of connected minions
    pub minions: Vec<Minion>,
}

impl Rattata {}

// start a server and wait for connections
pub fn start_rattata() -> Rattata {
    Rattata { minions: vec![] }
}
