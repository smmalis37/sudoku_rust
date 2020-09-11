use crate::{consts::*, solo_state::SoloState, sudoku_array::SudokuArray};
use druid::{
    widget::{Container, Either, Flex, Label, Widget, WidgetExt, WidgetId},
    BoxConstraints, Color, Data, Env, Event, EventCtx, KbKey, KeyEvent, LayoutCtx, LifeCycle,
    LifeCycleCtx, PaintCtx, Size, UpdateCtx,
};

#[derive(Clone, Data)]
pub struct Cell {
    pub value: Option<Num>,
    user_removed: SudokuArray<bool>,
    pub possibilities: SudokuArray<bool>,
    pub solo: SoloState<Num>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            user_removed: SudokuArray::new(false),
            possibilities: SudokuArray::new(true),
            solo: SoloState::None,
        }
    }
}

impl Cell {
    pub fn one_possibility(&self) -> Option<Num> {
        let mut ret = None;
        for (n, _) in self.possibility_iter().filter(|&(_, p)| p) {
            if ret.is_none() {
                ret = Some(n)
            } else {
                return None;
            }
        }
        ret
    }

    pub fn attempt_fill(&mut self) {
        if self.value.is_none() {
            if let SoloState::Solo(n) = self.solo {
                self.value = Some(n);
            } else if let Some(n) = self.one_possibility() {
                self.value = Some(n);
            }
        }
    }

    pub fn possibility_iter(&self) -> impl Iterator<Item = (Num, bool)> + '_ {
        self.possibilities
            .iter()
            .copied()
            .zip(self.user_removed.iter().copied())
            .enumerate()
            .map(|(i, (p, ur))| (i as Num + 1, p && !ur))
    }
}

pub struct GridSpace {
    root: WidgetId,
    up_target: WidgetId,
    down_target: WidgetId,
    display: Container<Cell>,
}

impl GridSpace {
    pub fn new(root: WidgetId, up_target: WidgetId, down_target: WidgetId) -> Self {
        Self {
            root,
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

    // TODO mess with alignments for better look?
    // TODO bold solos
    fn make_possibility_grid() -> impl Widget<Cell> {
        let mut column = Flex::column();
        for y in 0..SIZE {
            let mut row = Flex::row();
            for x in 0..SIZE {
                row.add_flex_child(
                    Label::dynamic(move |c: &Cell, _| {
                        let num = y * SIZE + x + 1;
                        if !c.possibilities[num] {
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
        let color = if focused {
            Color::rgb(0.6, 0.8, 1.0)
        } else if (data.value.is_some() && !data.possibilities[data.value.unwrap()])
            || matches!(data.solo, SoloState::Multiple)
            || data.possibility_iter().all(|(_, p)| !p)
        {
            Color::rgb(1.0, 0.6, 0.6)
        } else if data.value.is_none()
            && (matches!(data.solo, SoloState::Solo(_)) || data.one_possibility().is_some())
        {
            Color::rgb(0.7, 1.0, 0.7)
        } else {
            Color::WHITE
        };

        self.display.set_background(color);
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
                        .and_then(|c| c.to_digit(BASE as u32))
                        .map(|n| n as Num)
                        .filter(|&n| n != 0);

                    if let Some(num) = press {
                        if mods.ctrl() {
                            // TODO switch to shift?
                            if data.value.is_none() && data.possibilities[num] {
                                data.user_removed[num] = !data.user_removed[num];
                                ctx.submit_command(REGENERATE_SELECTOR.to(self.root));
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
            ctx.submit_command(REGENERATE_SELECTOR.to(self.root));
        }

        self.display.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Cell, env: &Env) {
        match event {
            LifeCycle::WidgetAdded => ctx.register_for_focus(),

            LifeCycle::FocusChanged(focused) => {
                self.set_background_color(data, *focused);
            }

            _ => {}
        }

        self.display.lifecycle(ctx, event, &data, env);
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Cell, data: &Cell, env: &Env) {
        self.set_background_color(data, ctx.has_focus());
        self.display.update(ctx, &old_data, &data, env);
    }

    fn layout(&mut self, ctx: &mut LayoutCtx, bc: &BoxConstraints, data: &Cell, env: &Env) -> Size {
        self.display.layout(ctx, bc, &data, env)
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &Cell, env: &Env) {
        self.display.paint(ctx, &data, env);
    }
}

fn radix_string<T>(n: T) -> String
where
    radix_fmt::Radix<T>: std::fmt::Display,
{
    format!("{:#}", radix_fmt::radix(n, BASE))
}
