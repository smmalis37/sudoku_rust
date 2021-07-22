use crate::*;
use iced_native::*;

pub(crate) trait Renderer:
    iced_native::Renderer
    + container::Renderer
    + text::Renderer
    + row::Renderer
    + column::Renderer
    + space::Renderer
{
}

impl<R> Renderer for R where
    R: iced_native::Renderer
        + container::Renderer
        + text::Renderer
        + row::Renderer
        + column::Renderer
        + space::Renderer
{
}

type M = <Sudoku as iced::Application>::Message;

pub(crate) struct Cell<'a, R>
where
    R: Renderer + 'a,
{
    // User controlled data
    value: Option<Num>,
    user_removed: SudokuArray<bool>,

    // Generated data
    possibilities: SudokuArray<bool>,
    _solo: SoloState<Num>,
    _in_invalid_group: bool,

    // UI
    contents: Option<Container<'a, M, R>>,
}

impl<'a, R> Default for Cell<'a, R>
where
    R: Renderer + 'a,
{
    fn default() -> Self {
        let mut s = Self {
            value: None,
            user_removed: SudokuArray::new(false),
            possibilities: SudokuArray::new(true),
            contents: None,
            _solo: SoloState::None,
            _in_invalid_group: false,
        };

        s.contents = Some(s.view());
        s
    }
}

impl<'a, R> From<Cell<'a, R>> for Element<'a, M, R>
where
    R: Renderer,
{
    fn from(cell: Cell<'a, R>) -> Element<'a, M, R> {
        Element::new(cell)
    }
}

impl<'a, R> Widget<M, R> for Cell<'a, R>
where
    R: Renderer + 'a,
{
    fn width(&self) -> Length {
        Widget::<M, R>::width(self.contents.as_ref().unwrap())
    }

    fn height(&self) -> Length {
        Widget::<M, R>::height(self.contents.as_ref().unwrap())
    }

    fn layout(
        &self,
        renderer: &R,
        limits: &iced_native::layout::Limits,
    ) -> iced_native::layout::Node {
        Widget::<M, R>::layout(self.contents.as_ref().unwrap(), renderer, limits)
    }

    fn draw(
        &self,
        renderer: &mut R,
        defaults: &R::Defaults,
        layout: iced_native::Layout<'_>,
        cursor_position: Point,
        viewport: &Rectangle,
    ) -> R::Output {
        Widget::<M, R>::draw(
            self.contents.as_ref().unwrap(),
            renderer,
            defaults,
            layout,
            cursor_position,
            viewport,
        )
    }

    fn hash_layout(&self, state: &mut iced_native::Hasher) {
        Widget::<M, R>::hash_layout(self.contents.as_ref().unwrap(), state)
    }
}

impl<'a, R> Cell<'a, R>
where
    R: Renderer + 'a,
{
    fn view(&self) -> Container<'a, M, R> {
        let content: Element<'a, M, R> = match self.value {
            Some(_) => self.make_value_text().into(),
            None => self.make_possibility_grid().into(),
        };

        Container::new(content).center_x().center_y() //.style(Theme)
    }

    fn make_value_text(&self) -> Text<R> {
        Text::new(self.value.unwrap().to_string())
            .size(48)
            .horizontal_alignment(HorizontalAlignment::Center)
            .vertical_alignment(VerticalAlignment::Center)
    }

    fn make_possibility_grid(&self) -> Column<'a, M, R> {
        let mut column = Column::new().align_items(Align::Center);

        for y in 0..SIZE {
            let mut row = Row::new().height(Length::Fill).align_items(Align::Center);

            for x in 0..SIZE {
                let num = y * SIZE + x + 1;
                let s = if !self.possibilities[num] {
                    String::new()
                } else if self.user_removed[num] {
                    "█".to_string()
                } else {
                    radix_string(num)
                };

                row = row.push(
                    Text::new(s)
                        .size(14)
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

    pub(crate) fn width(mut self, width: Length) -> Self {
        self.contents = Some(self.contents.take().unwrap().width(width));
        self
    }
}

struct Theme;

impl iced::container::StyleSheet for Theme {
    fn style(&self) -> iced::container::Style {
        iced::container::Style {
            border_width: 1.0,
            border_color: [0.75, 0.75, 0.75].into(),
            background: Some(Background::Color(Color::WHITE)),
            ..Default::default()
        }
    }
}

fn radix_string<T>(n: T) -> String
where
    radix_fmt::Radix<T>: std::fmt::Display,
{
    format!("{:#}", radix_fmt::radix(n, BASE))
}
