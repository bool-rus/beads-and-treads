#![windows_subsystem = "windows"]
mod reimport;
mod model;
mod ui;
mod wrapper;
mod io;
mod message;
mod service;

use reimport::*;
use message::Message;
use ui::*;
use service::AppService;
use model::{Model, Color};
use std::sync::Arc;

fn default_colors() -> Vec<Color> {
    vec![
        Color { r: 0x61 ,   g: 0x00,    b: 0x00 },
        Color { r: 0xff,    g: 0x00,    b: 0x88 },
        Color { r: 0x98,    g: 0x02,    b: 0x2f },
        Color { r: 0xcf,    g: 0x32,    b: 0x00 },
        Color { r: 0xff,    g: 0x32,    b: 0x32 },
        Color { r: 0xfd,    g: 0x8c,    b: 0x0e },
        Color { r: 0xff,    g: 0xe6,    b: 0x11 },
        Color { r: 0xff,    g: 0xfc,    b: 0x72 },
        Color { r: 0x88,    g: 0x0c,    b: 0x14 },
        Color { r: 0xb0,    g: 0x5e,    b: 0x07 },
        Color { r: 0x29,    g: 0x13,    b: 0x9c },
        Color { r: 0x3f,    g: 0x9b,    b: 0xe3 },
        Color { r: 0x69,    g: 0xc7,    b: 0xac },
        Color { r: 0x9a,    g: 0xcc,    b: 0xb0 },
        Color { r: 0x49,    g: 0x8c,    b: 0x55 },
        Color { r: 0x00,    g: 0xb1,    b: 0x5a },
        Color { r: 0x3e,    g: 0xe0,    b: 0x19 },
        Color { r: 0x8d,    g: 0xe4,    b: 0x6f },
        Color { r: 0x8c,    g: 0x62,    b: 0xd3 },
        Color { r: 0xc8,    g: 0xb5,    b: 0xff },
        Color { r: 0xdd,    g: 0xdd,    b: 0xdd },
        Color { r: 0x90,    g: 0x93,    b: 0x9e },
        Color { r: 0x00,    g: 0x00,    b: 0x00 },
    ]
}

struct App {
    service: AppService,
    top_menu: TopMenu,
    grid_plate: GridPlate<Model<Color>>,
    right_panel: RightPanel,
    right_menu: RightMenu,
    left_menu: LeftMenu,
    left_panel: LeftPanel,
}

impl Default for App {
    fn default() -> Self {
        let model = default_colors().into_iter().fold(Model::default(), |mut model, color| {
            model.add_color(color);
            model
        });
        let service = AppService::new(model.clone());
        let model = Arc::new(model);
        Self {
            service,
            top_menu: TopMenu::new(model.clone()),
            grid_plate: GridPlate::new(model.clone()),
            right_panel: RightPanel::new(model.clone()),
            right_menu: RightMenu::default(),
            left_menu: LeftMenu::default(),
            left_panel: Default::default(),
        }
    }
}

impl App {
    fn update_children(&mut self, message: Message) {
        self.top_menu.update(message.clone().into());
        self.right_menu.update(message.clone().into());
        self.left_menu.update(message.clone().into());
        self.grid_plate.update(message.clone().into());
        self.left_panel.update(message.clone().into());
        self.right_panel.update(message.clone().into());
    }
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Default::default()
    }
    fn title(&self) -> String {
        "Beads and threads by Bool".into()
    }
    fn update(&mut self, message: Message) {
        if let Some(service_msg) = self.service.process(message.clone().into()) {
            self.update_children(service_msg);
        }
        self.update_children(message.clone());
    }

    fn view(&mut self) -> Element<'_, Message> {
        let top = Container::new(self.top_menu.view().map(From::from))
            .height(Length::Units(30));
        let bottom = Container::new(Text::new(""));
        let left = Container::new(self.left_menu.view().map(From::from))
            .width(Length::Units(30));
        let right = Container::new(self.right_menu.view().map(From::from))
            .width(Length::Units(25));
        let content = Container::new(self.grid_plate.view().map(From::from));
        let row = Row::new().spacing(5)
            .push(Element::new(ui::MouseListener(Message::MouseRelease)))
            .width(Length::Fill)
            .height(Length::Fill)
            .push(left)
            .push(self.left_panel.view().map(From::from))
            .push(content.height(Length::Fill).width(Length::Fill))
            .push(self.right_panel.view().map(From::from))
            .push(right);
        Column::new().height(Length::Fill).spacing(5)
            .push(top)
            .push(row)
            .push(bottom).into()

    }
}

fn main() {
    App::run(Settings {
        window: iced::window::Settings {
            size: (550, 480),
            resizable: true,
            decorations: true,
            ..Default::default()
        },
        flags: (),
        default_font: None,
        antialiasing: false,
        ..Default::default()
    }).unwrap()
}
