use crate::prelude::*;
use iced_native::{
    keyboard::{KeyCode, Modifiers},
    *,
};

pub(crate) trait Renderer:
    container::Renderer<Style: From<Theme>> + text::Renderer + row::Renderer + column::Renderer
{
}

impl<R> Renderer for R where
    R: container::Renderer<Style: From<Theme>> + text::Renderer + row::Renderer + column::Renderer
{
}

pub(crate) struct Cell<'a, R: Renderer + 'a> {
    contents: Container<'a, M, R>,
    s: &'a mut State,
}

pub(crate) struct State {
    value: Option<Num>,
    user_removed: SudokuArray<bool>,
    is_focused: bool,
    pub g: Generated,
}

pub(crate) struct Generated {
    pub possibilities: SudokuArray<bool>,
    pub solo: SoloState<Num>,
    pub in_invalid_group: bool,
}

impl Default for State {
    fn default() -> Self {
        Self {
            value: None,
            user_removed: SudokuArray::new(false),
            is_focused: false,
            g: Default::default(),
        }
    }
}

impl Default for Generated {
    fn default() -> Self {
        Self {
            possibilities: SudokuArray::new(true),
            in_invalid_group: false,
            solo: SoloState::None,
        }
    }
}

impl<'a, R: Renderer + 'a> Cell<'a, R> {
    pub(crate) fn new(s: &'a mut State, l: Length) -> Self {
        Self {
            contents: Self::view(s).width(l).height(l),
            s,
        }
    }

    fn view(s: &State) -> Container<'a, M, R> {
        let content: Element<'a, M, R> = match s.value {
            Some(n) => Self::make_value_text(n).into(),
            None => Self::make_possibility_grid(s).into(),
        };

        Container::new(content)
            .center_x()
            .center_y()
            .style(Theme(s.bg_color()))
    }

    fn make_value_text(n: Num) -> Text<R> {
        Text::new(n.to_string())
            .size(48) // TODO: look into flexing text size
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
    }

    // TODO bold solos
    fn make_possibility_grid(s: &State) -> Column<'a, M, R> {
        let mut column = Column::new().align_items(Align::Center);

        for y in 0..SIZE {
            let mut row = Row::new().height(Length::Fill).align_items(Align::Center);

            for x in 0..SIZE {
                let num = y * SIZE + x + 1;
                let text = if !s.g.possibilities[num] {
                    String::new()
                } else if s.user_removed[num] {
                    "█".to_string()
                } else {
                    radix_string(num)
                };

                row = row.push(
                    Text::new(text)
                        .size(14) // TODO: look into flexing text size
                        .color([0.5, 0.5, 0.5])
                        .width(Length::Fill)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center),
                );
            }

            column = column.push(row);
        }

        column
    }
}

impl State {
    fn bg_color(&self) -> Color {
        const RED: Color = Color::from_rgb(1.0, 0.6, 0.6);
        const GREEN: Color = Color::from_rgb(0.7, 1.0, 0.7);
        const BLUE: Color = Color::from_rgb(0.7, 0.85, 1.0);
        // TODO gradients
        const BLUEGREEN: Color = Color::from_rgb(0.7, 0.925, 0.85);
        const BLUERED: Color = Color::from_rgb(0.85, 0.725, 0.8);

        let red = (self.value.is_some() && !self.g.possibilities[self.value.unwrap()])
            || matches!(self.g.solo, SoloState::Multiple)
            || self.possibility_iter().next().is_none()
            || self.g.in_invalid_group;

        let green = self.infer_value().is_some();
        let blue = self.is_focused;

        match (blue, green, red) {
            (false, false, false) => Color::WHITE,
            (true, false, false) => BLUE,
            (false, true, false) => GREEN,
            (false, _, true) => RED,
            (true, true, false) => BLUEGREEN,
            (true, _, true) => BLUERED,
        }
    }

    pub(crate) fn attempt_fill(&mut self) -> bool {
        if let Some(n) = self.infer_value() {
            self.value = Some(n);
            true
        } else {
            false
        }
    }

    fn infer_value(&self) -> Option<Num> {
        if self.value.is_none() {
            if let SoloState::Solo(n) = self.g.solo {
                return Some(n);
            } else if let Some(n) = self.one_possibility() {
                return Some(n);
            }
        }
        None
    }

    fn one_possibility(&self) -> Option<Num> {
        let mut possibilities = self.possibility_iter();
        let res = possibilities.next();

        if possibilities.next().is_some() {
            None
        } else {
            res
        }
    }

    pub(crate) fn possibility_iter(&self) -> impl Iterator<Item = Num> + '_ {
        self.g
            .possibilities
            .enumerate()
            .zip(self.user_removed.enumerate())
            .filter_map(|((i, &p), (_i, &ur))| (p && !ur).then(|| i))
    }

    fn handle_possibility(&mut self, c: KeyCode) -> bool {
        if let Some(n) = to_digit(c) {
            if self.value.is_none() && self.g.possibilities[n] {
                self.user_removed[n] = !self.user_removed[n];
                return true;
            }
        }
        false
    }

    pub(crate) const fn value(&self) -> Option<Num> {
        self.value
    }
}

impl<'a, R: Renderer> Widget<M, R> for Cell<'a, R> {
    fn width(&self) -> Length {
        Widget::width(&self.contents)
    }

    fn height(&self) -> Length {
        Widget::height(&self.contents)
    }

    fn layout(&self, renderer: &R, limits: &layout::Limits) -> layout::Node {
        self.contents.layout(renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut R,
        defaults: &R::Defaults,
        layout: Layout,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> R::Output {
        self.contents
            .draw(renderer, defaults, layout, cursor_position, viewport)
    }

    fn hash_layout(&self, state: &mut Hasher) {
        self.contents.hash_layout(state);
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout,
        cursor: Point,
        _: &R,
        _: &mut dyn Clipboard,
        messages: &mut Vec<M>,
    ) -> event::Status {
        // TODO add arrow key support
        if let Event::Keyboard(e) = event {
            if self.s.is_focused {
                let new_value = match e {
                    keyboard::Event::KeyPressed {
                        key_code: KeyCode::Backspace | KeyCode::Delete,
                        ..
                    } => None,

                    keyboard::Event::KeyPressed {
                        key_code,
                        modifiers: Modifiers { control: false, .. },
                    } if to_digit(key_code).is_some() => to_digit(key_code),

                    keyboard::Event::KeyPressed {
                        key_code,
                        modifiers: Modifiers { control: true, .. },
                    } if self.s.handle_possibility(key_code) => {
                        messages.push(Regen);
                        self.s.value
                    }

                    _ => self.s.value,
                };

                if self.s.value != new_value {
                    self.s.value = new_value;
                    messages.push(Regen);
                }

                event::Status::Captured
            } else {
                event::Status::Ignored
            }
        } else {
            match event {
                Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                    self.s.is_focused = layout.bounds().contains(cursor);
                    if self.s.is_focused {
                        messages.push(Redraw);
                    }
                    event::Status::Captured
                }
                _ => event::Status::Ignored,
            }
        }
    }
}

pub(crate) struct Theme(Color);

impl iced::container::StyleSheet for Theme {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            border_width: 1.0,
            border_color: Color::from_rgb(0.75, 0.75, 0.75),
            background: Some(Background::Color(self.0)),
            ..Default::default()
        }
    }
}

impl<'a, R: Renderer> From<Cell<'a, R>> for Element<'a, M, R> {
    fn from(cell: Cell<'a, R>) -> Element<'a, M, R> {
        Element::new(cell)
    }
}

// TODO support arbitrary bases
const fn to_digit(k: KeyCode) -> Option<Num> {
    match k {
        KeyCode::Key1 | KeyCode::Numpad1 => Some(1),
        KeyCode::Key2 | KeyCode::Numpad2 => Some(2),
        KeyCode::Key3 | KeyCode::Numpad3 => Some(3),
        KeyCode::Key4 | KeyCode::Numpad4 => Some(4),
        KeyCode::Key5 | KeyCode::Numpad5 => Some(5),
        KeyCode::Key6 | KeyCode::Numpad6 => Some(6),
        KeyCode::Key7 | KeyCode::Numpad7 => Some(7),
        KeyCode::Key8 | KeyCode::Numpad8 => Some(8),
        KeyCode::Key9 | KeyCode::Numpad9 => Some(9),
        _ => None,
    }
}

fn radix_string<T>(n: T) -> String
where
    radix_fmt::Radix<T>: std::fmt::Display,
{
    format!("{:#}", radix_fmt::radix(n, BASE))
}
