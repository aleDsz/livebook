extern crate self as elixirkit;

pub struct API {
    release: Release,
}

impl API {
    pub fn new(name: &str) -> Self {
        let release = Release::new(name);
        Self { release }
    }

    pub fn stop(&self) -> i32 {
        self.release.exit_code()
    }

    pub fn has_exited(&self) -> bool {
        self.release.has_exited()
    }

    pub fn wait_for_exit(&self) -> i32 {
        self.release.exit_code()
    }

    pub fn publish(&self, name: &str, data: &str) {
        self.release.send(data);
    }

    pub fn subscribe(&self) {
        // self.release.send("");
    }
}

struct Release {
    listener: std::net::TcpListener,
    conn: std::net::TcpStream,
    command: std::process::Command,
}

impl Release {
    pub fn new(name: &str) -> Self {}

    pub fn has_exited(&self) -> bool {
        false
    }

    pub fn exit_code(&self) -> i32 {
        0
    }

    pub fn send(&self, message: &str) {
        //
    }
}
