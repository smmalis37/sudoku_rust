use crate::{consts::*, solo_state::SoloState, sudoku_array::SudokuArray};
use druid::{
    widget::{BackgroundBrush, Container, Either, Flex, Label, Widget, WidgetExt, WidgetId},
    BoxConstraints, Color, Data, Env, Event, EventCtx, KbKey, KeyEvent, LayoutCtx, LifeCycle,
    LifeCycleCtx, LinearGradient, PaintCtx, Size, UnitPoint, UpdateCtx,
};

#[derive(Clone, Data)]
pub struct Cell {
    // User controlled data
    value: Option<Num>,
    user_removed: SudokuArray<bool>,
    // Generated data
    pub g: CellGeneratedData,
}

#[derive(Clone, Data)]
pub struct CellGeneratedData {
    pub possibilities: SudokuArray<bool>,
    pub solo: SoloState<Num>,
    pub in_invalid_group: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            user_removed: SudokuArray::new(false),
            g: Default::default(),
        }
    }
}

impl Default for CellGeneratedData {
    fn default() -> Self {
        Self {
            possibilities: SudokuArray::new(true),
            solo: SoloState::None,
            in_invalid_group: false,
        }
    }
}

impl Cell {
    pub fn value(&self) -> Option<Num> {
        self.value
    }

    pub fn attempt_fill(&mut self) -> bool {
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
        let mut possibilities = self.possibility_iter().filter(|&(_, p)| p);
        let res = possibilities.next().map(|(n, _)| n);

        if possibilities.next().is_some() {
            None
        } else {
            res
        }
    }

    pub fn possibility_iter(&self) -> impl Iterator<Item = (Num, bool)> + '_ {
        self.g
            .possibilities
            .enumerate()
            .zip(self.user_removed.enumerate())
            .map(|((i, &p), (_i, &ur))| (i, p && !ur))
    }
}

pub struct GridSpace {
    up_target: WidgetId,
    down_target: WidgetId,
    display: Container<Cell>,
}

impl GridSpace {
    pub fn new(up_target: WidgetId, down_target: WidgetId) -> Self {
        Self {
            up_target,
            down_target,
            display: Either::new(
                |c: &Cell, _| c.value.is_some(),
                Self::make_value_label(),
                Self::make_possibility_grid(),
            )
            .center()
            .background(Color::WHITE)
            .border(Color::grey(0.5), 0.5),
        }
    }

    fn make_value_label() -> impl Widget<Cell> {
        Label::dynamic(|c: &Cell, _| c.value.map(radix_string).unwrap_or_default())
            .with_text_size(48.0) // TODO: look into flexing text size
            .with_text_color(Color::BLACK)
    }

    // TODO bold solos
    fn make_possibility_grid() -> impl Widget<Cell> {
        let mut column = Flex::column();
        for y in 0..SIZE {
            let mut row = Flex::row();
            for x in 0..SIZE {
                let num = y * SIZE + x + 1;
                row.add_flex_child(
                    Label::dynamic(move |c: &Cell, _| {
                        if !c.g.possibilities[num] {
                            String::new()
                        } else if c.user_removed[num] {
                            "█".to_string()
                        } else {
                            radix_string(num)
                        }
                    })
                    .with_text_size(12.0) // TODO: look into flexing text size
                    .with_text_color(Color::grey(0.5))
                    .center(),
                    1.0,
                );
            }
            column.add_flex_child(row, 1.0);
        }
        column
    }

    fn set_background_color(&mut self, data: &Cell, focused: bool) {
        const BLUE: Color = Color::rgb8(178, 217, 255);
        const GREEN: Color = Color::rgb8(178, 255, 178);
        const RED: Color = Color::rgb8(255, 153, 153);

        // TODO constify gradients
        let blue_green: LinearGradient = LinearGradient::new(
            UnitPoint::TOP_LEFT,
            UnitPoint::BOTTOM_RIGHT,
            [BLUE, GREEN].as_ref(),
        );
        let blue_red: LinearGradient = LinearGradient::new(
            UnitPoint::TOP_LEFT,
            UnitPoint::BOTTOM_RIGHT,
            [BLUE, RED].as_ref(),
        );

        let blue = focused;
        let green = data.infer_value().is_some();
        let red = (data.value.is_some() && !data.g.possibilities[data.value.unwrap()])
            || matches!(data.g.solo, SoloState::Multiple)
            || data.possibility_iter().all(|(_, p)| !p)
            || data.g.in_invalid_group;

        let brush: BackgroundBrush<_> = match (blue, green, red) {
            (false, false, false) => Color::WHITE.into(),
            (true, false, false) => BLUE.into(),
            (false, true, false) => GREEN.into(),
            (false, _, true) => RED.into(),
            (true, true, false) => blue_green.into(),
            (true, _, true) => blue_red.into(),
        };

        self.display.set_background(brush);
    }
}

impl Widget<Cell> for GridSpace {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Cell, env: &Env) {
        // TODO tab
        let mut new_val = data.value;
        match event {
            Event::KeyDown(KeyEvent { key, mods, .. }) => match key {
                KbKey::Character(c) => {
                    let press = c
                        .chars()
                        .last()
                        .and_then(|c| c.to_digit(BASE.into()))
                        .map(|n| n as Num)
                        .filter(|&n| n != 0);

                    if let Some(num) = press {
                        if mods.ctrl() {
                            // TODO switch to shift?
                            if data.value.is_none() && data.g.possibilities[num] {
                                data.user_removed[num] = !data.user_removed[num];
                                ctx.submit_notification(REGENERATE_SELECTOR);
                            }
                        } else {
                            new_val = press;
                        }
                    }
                }

                // TODO add others?
                KbKey::Backspace | KbKey::Delete => {
                    new_val = None;
                }

                KbKey::ArrowLeft => ctx.focus_prev(),
                KbKey::ArrowRight => ctx.focus_next(),
                KbKey::ArrowUp => ctx.set_focus(self.up_target),
                KbKey::ArrowDown => ctx.set_focus(self.down_target),

                _ => {}
            },
            Event::MouseDown(_) => ctx.request_focus(),
            _ => {}
        };

        if new_val != data.value {
            data.value = new_val;
            ctx.submit_notification(REGENERATE_SELECTOR);
        }

        self.display.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Cell, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => ctx.register_for_focus(),
            LifeCycle::FocusChanged(focused) => {
                self.set_background_color(data, *focused);
                ctx.request_paint();
            }
            _ => {}
        }

        self.display.lifecycle(ctx, event, data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Cell, data: &Cell, env: &Env) {
        self.set_background_color(data, ctx.has_focus());
        ctx.request_paint();
        self.display.update(ctx, old_data, data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Cell, env: &Env) -> Size {
        self.display.layout(ctx, bc, data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Cell, env: &Env) {
        self.display.paint(ctx, data, env);
    }
}

fn radix_string<T>(n: T) -> String
where
    radix_fmt::Radix<T>: std::fmt::Display,
{
    format!("{:#}", radix_fmt::radix(n, BASE))
}
