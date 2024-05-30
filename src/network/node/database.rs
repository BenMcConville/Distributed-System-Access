pub struct DataBase {
    name: String,
}

impl DataBase {
    pub fn new(name: String) -> DataBase {
        DataBase { name: name }
    }
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
