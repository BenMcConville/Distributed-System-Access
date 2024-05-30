use super::network;
use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub app_network: network::Network,
    pub node_input: String,            // the currently being edited json key.
    pub value_input: String,           // the currently being edited json value.
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
    pub currently_editing: Option<CurrentlyEditing>, // the optional state containing which of the key or value pair the user is editing. It is an option, because when the user is not directly editing a key-value pair, this will be set to `None`.
    pub current_selection: Option<usize>,
    pub current_selection_name: Option<String>,
    pub update_node: bool,
}

fn load_nodes(network: &mut network::Network) {
    network.add_server(String::from("node03"), String::from("sr01"));
    network.add_server(String::from("node04"), String::from("sr02"));
    network.add_data_base(String::from("node01"), String::from("db01"));
    network.add_data_base(String::from("node02"), String::from("db02"));
}

impl App {
    pub fn new() -> App {
        let mut network = network::Network::new();
        load_nodes(&mut network);
        App {
            app_network: network,
            node_input: String::new(),
            value_input: String::new(),
            current_screen: CurrentScreen::Main,
            currently_editing: Some(CurrentlyEditing::Key),
            current_selection: Some(0),
            current_selection_name: None,
            update_node: false,
        }
    }
    pub fn get_servers(&self) -> Vec<(&String, bool)> {
        self.app_network.get_servers()
    }
    pub fn get_databases(&self) -> Vec<(&String, bool)> {
        self.app_network.get_databases()
    }

    pub fn toggle_selected_node(&mut self) {
        let selected_node_name = self.current_selection_name.unwrap();
        //     String::from(self.app_network.get_nodes()[self.current_selection.unwrap()].0);
        self.app_network.toggle_node(&selected_node_name);
    }

    pub fn move_up(&mut self) {
        match self.current_selection {
            Some(position) => {
                let size = self.app_network.get_number_of_nodes();
                self.current_selection = {
                    if (position == 0) {
                        Some(size - 1)
                    } else {
                        Some(position - 1)
                    }
                }
            }
            None => {
                if (self.app_network.get_number_of_nodes() > 0) {
                    self.current_selection = Some(0);
                }
            }
        }
    }
    pub fn move_down(&mut self) {
        match self.current_selection {
            Some(position) => {
                let size = self.app_network.get_number_of_nodes();
                self.current_selection = Some((position + 1) % size);
            }
            None => {
                if (self.app_network.get_number_of_nodes() > 0) {
                    self.current_selection = Some(0);
                }
            }
        }
    }

    pub fn toggle_editing(&mut self) {
        if let Some(edit_mode) = &self.currently_editing {
            match edit_mode {
                CurrentlyEditing::Key => self.currently_editing = Some(CurrentlyEditing::Value),
                CurrentlyEditing::Value => self.currently_editing = Some(CurrentlyEditing::Key),
            };
        } else {
            self.currently_editing = Some(CurrentlyEditing::Key);
        }
    }
}
