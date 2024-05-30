pub struct Server {
    name: String,
}

impl Server {
    pub fn new(name: String) -> Server {
        Server { name: name }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
