pub mod node;

pub struct Network {
    network_name: String,
    network_nodes: Vec<node::Node>,
}

impl Network {
    pub fn new() -> Network {
        Network {
            network_name: String::from("test_Network"),
            network_nodes: vec![],
        }
    }
    pub fn get_name(&self) -> &String {
        &self.network_name
    }
    pub fn get_number_of_nodes(&self) -> usize {
        self.network_nodes.len()
    }
    pub fn get_nodes(&self) -> Vec<(&String, bool)> {
        let list = self
            .network_nodes
            .iter()
            .map(|n| (n.get_node_name(), n.get_node_status()))
            .collect();
        list
    }

    pub fn get_servers(&self) -> Vec<(&String, bool)> {
        let list = self
            .network_nodes
            .iter()
            .filter(|&n| matches!(n.get_node_type(), node::NodeTypes::ServerNode(..)))
            .map(|n| (n.get_node_name(), n.get_node_status()))
            .collect();
        list
    }
    pub fn get_databases(&self) -> Vec<(&String, bool)> {
        let list = self
            .network_nodes
            .iter()
            .filter(|&n| matches!(n.get_node_type(), node::NodeTypes::DataBaseNode(..)))
            .map(|n| (n.get_node_name(), n.get_node_status()))
            .collect();
        list
    }

    pub fn add_server(&mut self, node_input_name: String, server_input_name: String) {
        self.network_nodes
            .push(node::Node::new_server(node_input_name, server_input_name));
    }
    pub fn add_data_base(&mut self, node_input_name: String, database_input_name: String) {
        self.network_nodes.push(node::Node::new_database(
            node_input_name,
            database_input_name,
        ));
    }
    pub fn remove_node(&mut self, node_input: &String) -> bool {
        for (index, node) in self.network_nodes.iter().enumerate() {
            if node.get_node_name() == node_input {
                self.network_nodes.remove(index);
                return true;
            }
        }
        println!("No value found");
        return false;
    }
    pub fn toggle_node(&mut self, node_input: &String) -> bool {
        for (index, node) in self.network_nodes.iter().enumerate() {
            if node.get_node_name() == node_input {
                self.network_nodes[index].toggle_node_status();
                return true;
            }
        }
        println!("No value found");
        return false;
    }
}
