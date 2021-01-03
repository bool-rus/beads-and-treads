use crate::reimport::*;
use iced::container;
use button::Style;
use iced::{Color, Background};

pub struct ColorButton(pub Color);

impl button::StyleSheet for ColorButton {
    fn active(&self) -> Style {
        Style {
            background: Some(Background::Color(self.0.clone())),
            ..Default::default()
        }
    }

    fn disabled(&self) -> Style {
        Style {
            border_width: 2.0,
            border_color: Color::BLACK,
            ..self.active()
        }
    }
}

pub struct FSMenuItem;

pub struct ToggledOn;

pub struct Colored(pub Color);


impl slider::StyleSheet for Colored {
    fn active(&self) -> slider::Style {
        slider::Style {
            rail_colors: (Color {r: 1.0, g:0.0 ,b:0.0, a: 1.0}, Color {r:0.0, g: 1.0, b: 0.0, a: 1.0}),
            handle: slider::Handle {
                shape: slider::HandleShape::Rectangle {
                    width: 8,
                    border_radius: 4.0,
                },
                color: self.0,
                border_color: Color::from_rgb(0.6, 0.6, 0.6),
                border_width: 1.0,
            },
        }
    }

    fn hovered(&self) -> slider::Style {
        self.active()
    }

    fn dragging(&self) -> slider::Style {
        self.active()
    }
}

impl button::StyleSheet for ToggledOn {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color{
                r: 0.5,
                g: 0.5,
                b: 0.5,
                a: 1.0
            })),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }
}

impl button::StyleSheet for FSMenuItem {
    fn active(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: None,
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::BLACK,
            text_color: Color::BLACK,
        }
    }

    fn hovered(&self) -> Style {
        Style {
            shadow_offset: Default::default(),
            background: Some(Background::Color(Color::BLACK)),
            border_radius: 0.0,
            border_width: 0.0,
            border_color: Color::WHITE,
            text_color: Color::WHITE
        }
    }
}

impl container::StyleSheet for Colored {
    fn style(&self) -> container::Style {
        container::Style {
            background: Some(Background::Color(self.0)),
            ..Default::default()
        }
    }
}