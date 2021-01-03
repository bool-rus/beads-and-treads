use iced_native::{Widget, layout, Layout, event, Event, Clipboard, Hasher};
use iced_wgpu::{Primitive, Renderer, Defaults};
use iced::mouse::Interaction;
use iced::mouse;
use iced::{Background, Color, Element, Length, Point, Rectangle, Size};
use crate::wrapper::Wrappable;
use std::hash::Hash;
use iced_wgpu::triangle::{Mesh2D, Vertex2D};

pub struct ColorBox<T> {
    color: Color,
    width: Length,
    height: Length,
    press_message: Option<T>,
    over_message: Option<T>,
    border_color: Color,
}

impl <T> ColorBox<T> {
    pub fn new<C: Into<Color>>(color: C) -> Self {
        Self {
            color: color.into(),
            width: Length::Units(30),
            height: Length::Units(30),
            press_message: None,
            over_message: None,
            border_color: Color::BLACK,
        }
    }
    pub fn border_color(mut self, color: Color) -> Self {
        self.border_color = color;
        self
    }
    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: Length) -> Self {
        self.height = height;
        self
    }
    pub fn on_press(mut self, msg: T) -> Self {
        self.press_message = Some(msg);
        self
    }
    pub fn on_over(mut self, msg: T) -> Self {
        self.over_message = Some(msg);
        self
    }
}

impl<Message:Clone> Widget<Message, Renderer> for ColorBox<Message> {
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        layout::Node::new(limits.resolve(Size::ZERO))
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle
    ) -> (Primitive, Interaction) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(self.color.clone().into()),
                border_radius: 0.0,
                border_width: 1.0,
                border_color: self.border_color,
            },
            Interaction::Idle,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        self.color.wrap().hash(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor_position: Point,
        messages: &mut Vec<Message>,
        _renderer: &Renderer,
        _clipboard: Option<&dyn Clipboard>,
    ) -> event::Status {
        if let Some(ref msg) = self.press_message {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed (mouse::Button::Left)) => {
                    if layout.bounds().contains(cursor_position) {
                        messages.push(msg.clone())
                    }
                },
                _ => {}
            }
        };
        if let (Some(msg), Event::Mouse(mouse::Event::CursorMoved {..})) = (&self.over_message, event) {
            if layout.bounds().contains(cursor_position) {
                messages.push(msg.clone())
            }
        };
        event::Status::Ignored
    }

}

impl<'a, M: 'a + Clone> Into<Element<'a,M>> for ColorBox<M> {
    fn into(self) -> Element<'a,M> {
        Element::new(self)
    }
}

pub struct MouseListener<M>(pub M);


impl<Message: Clone> Widget<Message, Renderer> for MouseListener<Message> {
    fn width(&self) -> Length {
        Length::Units(0)
    }

    fn height(&self) -> Length {
        Length::Units(0)
    }

    fn layout(&self, _renderer: &Renderer, _limits: &layout::Limits) -> layout::Node {
        layout::Node::new(Size::ZERO)
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        _layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, Interaction) {
        (Primitive::None, Interaction::Idle)
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        0.hash(state)
    }

    fn on_event(&mut self,
                event: Event,
                _layout: Layout<'_>,
                _cursor_position: Point,
                messages: &mut Vec<Message>,
                _renderer: &Renderer,
                _clipboard: Option<&dyn Clipboard>) -> event::Status {
        if matches!(event, Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left))) {
            messages.push(self.0.clone())
        }
        event::Status::Ignored
    }
}
#[derive(Debug, Copy, Clone)]
pub enum Gradient {
    Hue,
    Saturation(f32),
    Light {
        hue: f32,
        sat: f32,
    },
}

impl Gradient {
    fn hue_gradient() -> Mesh2D {
        let color_r = [1.0, 0.0, 0.0, 1.0];
        let color_rg = [1.0,1.0,0.0,1.0];
        let color_g = [0.0, 1.0, 0.0, 1.0];
        let color_gb = [0.0, 1.0, 1.0, 1.0];
        let color_b = [0.0, 0.0, 1.0, 1.0];
        let color_br = [1.0, 0.0, 1.0, 1.0];
        let chunk = 1.0/6.0;
        Mesh2D {
            vertices: vec![
                Vertex2D { position: [0.0,      0.0],   color: color_r },
                Vertex2D { position: [0.0,      1.0],   color: color_r, },
                Vertex2D { position: [chunk,    0.0],   color: color_rg, },
                Vertex2D { position: [chunk,    1.0],   color: color_rg, },
                Vertex2D { position: [chunk*2.0, 0.0],  color: color_g, },
                Vertex2D { position: [2.0*chunk, 1.0],  color: color_g, },
                Vertex2D { position: [3.0*chunk, 0.0],  color: color_gb, },
                Vertex2D { position: [3.0*chunk, 1.0],  color: color_gb, },
                Vertex2D { position: [4.0*chunk, 0.0],  color: color_b, },
                Vertex2D { position: [4.0*chunk, 1.0],  color: color_b, },
                Vertex2D { position: [5.0*chunk, 0.0],  color: color_br, },
                Vertex2D { position: [5.0*chunk, 1.0],  color: color_br, },
                Vertex2D { position: [1.0,      0.0],   color: color_r, },
                Vertex2D { position: [1.0,      1.0],   color: color_r, },
            ],
            indices: vec![
                0, 1, 2,
                2, 3, 4,
                4, 5, 6,
                6, 7, 8,
                8, 9, 10,
                10, 11, 12,
                13, 12, 11,
                11, 10, 9,
                9, 8, 7,
                7, 6, 5,
                5, 4, 3,
                3, 2, 1,
            ],
        }
    }
    fn sat_gradient(hue: f32) -> Mesh2D {
        let gray = [0.5, 0.5, 0.5, 1.0];
        let hsl: colors::Hsl<colors::encoding::Srgb, _> = colors::Hsl::from_components(
            (colors::RgbHue::from_degrees(hue), 1.0, 0.5)
        );
        let (r,g,b) = colors::Srgb::from(hsl).into_components();
        let color = [r, g, b, 1.0];
        Mesh2D {
            vertices: vec![
                Vertex2D { position: [0.0, 0.0], color: gray },
                Vertex2D { position: [0.0, 1.0], color: gray },
                Vertex2D { position: [1.0, 0.0], color: color },
                Vertex2D { position: [1.0, 1.0], color: color },
            ],
            indices: vec![0, 1, 2, 1, 2, 3],
        }
    }
    fn light_gradient(hue: f32, sat: f32) -> Mesh2D {
        let hsl: colors::Hsl<colors::encoding::Srgb, _> = colors::Hsl::from_components(
            (colors::RgbHue::from_degrees(hue), sat, 0.5)
        );
        let (r,g,b) = colors::Srgb::from(hsl).into_components();
        let color = [r, g, b, 1.0];
        let black = [0.0, 0.0, 0.0, 1.0];
        let white = [1.0;4];
        Mesh2D {
            vertices: vec![
                Vertex2D {position: [0.0, 0.0], color: black},
                Vertex2D {position: [0.0, 1.0], color: black},
                Vertex2D {position: [0.5, 0.0], color},
                Vertex2D {position: [0.5, 1.0], color},
                Vertex2D {position: [1.0, 0.0], color: white},
                Vertex2D {position: [1.0, 1.0], color: white},
            ],
            indices: vec![0,1,2, 2,3,4, 1,2,3, 3,4,5]
        }
    }
}

impl<Message> Widget<Message, Renderer> for Gradient
{
    fn width(&self) -> Length {
        Length::Fill
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let size = limits.width(Length::Fill).height(Length::Units(20)).resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn hash_layout(&self, _state: &mut Hasher) {}

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) -> (Primitive, Interaction) {
        let b = layout.bounds();

        let w = b.width;
        let h = b.height;
        let Mesh2D {vertices, indices} = match self {
            &Gradient::Hue => Self::hue_gradient(),
            &Gradient::Saturation(hue) => Self::sat_gradient(hue),
            &Gradient::Light {hue, sat} => Self::light_gradient(hue, sat),
        };
        let vertices: Vec<_> = vertices.into_iter().map(|Vertex2D{ mut position, color}| {
            position[0] *= w;
            position[1] *= h;
            Vertex2D { position, color }
        }).collect();

        (Primitive::Mesh2D {
                    size: iced::Size {width: b.x, height: b.y},
                    buffers: Mesh2D {vertices, indices },
                },
            Interaction::Idle,
        )
    }
}
