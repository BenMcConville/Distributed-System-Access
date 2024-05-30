use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    Frame,
};

use crate::{
    app::{App, CurrentScreen, CurrentlyEditing},
    network,
};

pub fn ui(f: &mut Frame, app: &App) {
    // Create the layout sections.
    //
    // When displaying list options:
    match app.current_screen {
        CurrentScreen::Main => main_page(f, app),
        CurrentScreen::Editing => {
            main_page(f, app);
            popup(f, app);
        }
        _ => {
            println!("Nothing Here");
        }
    }
}
fn main_page(f: &mut Frame, app: &App) {
    let screen = Layout::default()
        .constraints([Constraint::Percentage(100)])
        .split(f.size());
    let screen_ui = Block::default()
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Rgb(41, 44, 50)));
    f.render_widget(screen_ui, screen[0]);

    let layer01 = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(20), Constraint::Percentage(80)])
        .split(screen[0]);
    let layer01_ui = Block::default().borders(Borders::ALL);

    let list = List::new(load_list_items(app)).block(Block::default().borders(Borders::ALL));

    f.render_widget(list, layer01[0]);
    // f.render_widget(&layer01_ui, layer01[0]);
    f.render_widget(&layer01_ui, layer01[1]);
}
fn popup(f: &mut Frame, app: &App) {
    let popup_block = Block::default()
        .title("Enter a new key-value pair")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::DarkGray));
    let area = centered_rect(60, 25, f.size());
    f.render_widget(popup_block, area);
}

fn append_list_elem<'a>(
    mut list_items: Vec<ListItem<'a>>,
    node_name: String,
    node_status: bool,
    selected: bool,
) -> Vec<ListItem<'a>> {
    if node_status {
        if selected {
            list_items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    " Online   ",
                    Style::default().fg(Color::Green).bg(Color::Yellow),
                ),
                Span::styled(node_name, Style::default().fg(Color::White)),
            ])));
            // if update_elem {
            //     current_node.toggle_status();
            // }
        } else {
            list_items.push(ListItem::new(Line::from(vec![
                Span::styled(" Online   ", Style::default().fg(Color::Green)),
                Span::styled(node_name, Style::default().fg(Color::White)),
            ])));
        }
    } else {
        if selected {
            list_items.push(ListItem::new(Line::from(vec![
                Span::styled(
                    " Offline  ",
                    Style::default().fg(Color::Red).bg(Color::Yellow),
                ),
                Span::styled(node_name, Style::default().fg(Color::White)),
            ])));
            // if update_elem {
            //     current_node.toggle_status();
            // }
        } else {
            list_items.push(ListItem::new(Line::from(vec![
                Span::styled(" Offline  ", Style::default().fg(Color::Red)),
                Span::styled(node_name, Style::default().fg(Color::White)),
            ])));
        }
    }
    list_items
}

fn load_list_items(app: &mut App) -> Vec<ListItem> {
    let mut list_items = Vec::<ListItem>::new();
    let mut counter: usize = 0;
    list_items.push(ListItem::new(Line::from(Span::styled(
        String::from("Servers:"),
        Style::default().fg(Color::White),
    ))));
    for (node_name, node_status) in app.get_servers().iter() {
        list_items = append_list_elem(
            list_items,
            String::from(*node_name),
            *node_status,
            counter == app.current_selection.unwrap(),
        );
        if counter == app.current_selection.unwrap() {
            app.current_selection_name = Some(String::from(*node_name));
        }
        counter += 1;
    }
    list_items.push(ListItem::new(Line::from(Span::styled(
        String::from("Database:"),
        Style::default().fg(Color::White),
    ))));
    for (node_name, node_status) in app.get_databases().iter() {
        list_items = append_list_elem(
            list_items,
            String::from(*node_name),
            *node_status,
            counter == app.current_selection.unwrap(),
        );
        counter += 1;
    }

    list_items
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}
