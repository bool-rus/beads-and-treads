mod reimport;
mod grid;
mod entities;
mod ui;
mod wrapper;
mod io;
mod beads;
mod message;

use reimport::*;
use grid::Grid;
use entities::Color;
use message::Message;
use ui::*;
use std::rc::Rc;
use std::cell::{RefCell, Cell};
use std::num::NonZeroUsize;
use crate::entities::{Schema, GridAction, Side};



struct App {
    grid: Rc<RefCell<Grid<Color>>>,
    top_menu: TopMenu,
    grid_plate: GridPlate,
    right_panel: RightPanel,
    right_menu: RightMenu,
    left_menu: LeftMenu,
    active_color: Color,
    left_panel: LeftPanel,
    mouse_hold: Rc<Cell<bool>>,
}

impl Default for App {
    fn default() -> Self {
        let grid = Rc::new(RefCell::new(Default::default()));
        let schema = Rc::new(Cell::new(Schema::FirstOffset));
        let mouse_hold = Rc::new(Cell::new(false));
        Self {
            grid: grid.clone(),
            top_menu: Default::default(),
            grid_plate: GridPlate::new(grid.clone(), schema.clone(), mouse_hold.clone()),
            right_panel: RightPanel::new(grid.clone(), schema.clone()),
            right_menu: RightMenu::default(),
            left_menu: LeftMenu::default(),
            active_color: Default::default(),
            mouse_hold,
            left_panel: Default::default(),
        }
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
        self.top_menu.update(message.into());
        self.right_menu.update(message.into());
        self.left_menu.update(message.into());
        self.grid_plate.update(message.into());
        self.left_panel.update(message.into());
        self.right_panel.update(message.into());
        match message {
            Message::TopMenu(TopMenuMessage::Palette(PaletteMessage::SetColor(color))) =>  {
                self.active_color = color
            },
            Message::LeftMenu(LeftMenuMessage::ShowResize) |
            Message::LeftPanel(LeftPanelMessage::GridAction(_)) => {
                let grid = self.grid.borrow();
                use LeftPanelMessage::*;
                self.left_panel.update(InputWidth(grid.width()));
                self.left_panel.update(InputHeight(grid.height()));
            },
            Message::Grid(GridMessage::GridClicked(row, col)) => {
                self.grid_plate.update(GridMessage::SetColor(row, col,self.active_color))
            },
            Message::LeftPanel(LeftPanelMessage::Resize(width, height)) => {
                if let (Some(width), Some(height)) =
                (NonZeroUsize::new(width), NonZeroUsize::new(height)) {
                    self.grid.borrow_mut().resize(width, height);
                }
            }
            Message::LeftPanel(LeftPanelMessage::FS(FilesMessage::Open)) => {
                if let Some(path) = self.left_panel.selected_path() {
                    let grid = crate::io::read(path).unwrap();
                    self.grid.borrow_mut().update_from_another(grid);
                }
            },
            Message::LeftPanel(LeftPanelMessage::FS(FilesMessage::Save)) => {
                if let Some(path) = self.left_panel.selected_path() {
                    crate::io::write(path, self.grid.borrow().as_table()).unwrap();
                }
            },
            _ => {}
        }
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
            .push(Element::new(ui::MouseListener::new(self.mouse_hold.clone())))
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
        },
        flags: (),
        default_font: None,
        antialiasing: false,
    });
}
