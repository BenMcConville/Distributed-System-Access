pub mod database;
pub mod server;
use self::database::DataBase;
use self::server::Server;

pub enum NodeTypes {
    ServerNode(Server),
    DataBaseNode(DataBase),
    None,
}

pub struct Node {
    node_name: String,
    node_status: bool,
    node_type_instance: NodeTypes,
}
impl Node {
    pub fn new_server(node_input_name: String, server_input_name: String) -> Node {
        Node {
            node_name: node_input_name,
            node_status: false,
            node_type_instance: NodeTypes::ServerNode(Server::new(server_input_name)),
        }
    }
    pub fn new_database(node_input_name: String, database_input_name: String) -> Node {
        Node {
            node_name: node_input_name,
            node_status: false,
            node_type_instance: NodeTypes::DataBaseNode(DataBase::new(database_input_name)),
        }
    }
    pub fn get_node_type(&self) -> &NodeTypes {
        &self.node_type_instance
    }
    pub fn get_node_name(&self) -> &String {
        &self.node_name
    }
    pub fn get_node_status(&self) -> bool {
        self.node_status
    }
    pub fn toggle_node_status(&mut self) {
        self.node_status = !self.node_status;
    }
}
