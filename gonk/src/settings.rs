use crate::{widgets::*, Frame, Input};
use gonk_core::Index;
use gonk_player::{default_device, devices};
use tui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders},
};

//TODO: Devices are not refreshed when new ones are connected.
pub struct Settings {
    pub devices: Index<&'static str>,
    pub current_device: String,
}

impl Settings {
    pub fn new() -> Self {
        let wanted_device = gonk_core::output_device();
        let devices = devices();
        let default = default_device().unwrap();

        let devices: Vec<&'static str> =
            devices.iter().map(|device| device.name.as_str()).collect();

        let current_device = if devices.iter().any(|name| *name == wanted_device) {
            wanted_device.to_string()
        } else {
            default.name.to_string()
        };

        Self {
            devices: Index::from(devices),
            current_device,
        }
    }
    pub fn update(&mut self) {
        let mut index = self.devices.index().unwrap_or(0);
        if index >= devices().len() {
            index = devices().len().saturating_sub(1);
        }

        self.devices = Index::new(
            devices()
                .iter()
                .map(|device| device.name.as_str())
                .collect(),
            Some(index),
        );
    }
}

impl Input for Settings {
    fn up(&mut self) {
        self.devices.up();
    }

    fn down(&mut self) {
        self.devices.down();
    }

    fn left(&mut self) {}

    fn right(&mut self) {}
}

pub fn draw(settings: &mut Settings, area: Rect, f: &mut Frame) {
    let items: Vec<ListItem> = settings
        .devices
        .data
        .iter()
        .map(|name| {
            if *name == settings.current_device {
                ListItem::new(*name)
            } else {
                ListItem::new(*name).style(Style::default().add_modifier(Modifier::DIM))
            }
        })
        .collect();

    let list = List::new(&items)
        .block(
            Block::default()
                .title("─Output Device")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded),
        )
        .style(Style::default().fg(Color::White))
        .highlight_style(Style::default())
        .highlight_symbol("> ");

    let mut state = ListState::default();
    state.select(settings.devices.index());

    f.render_stateful_widget(list, area, &mut state);
}
