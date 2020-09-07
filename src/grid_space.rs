use crate::{consts::*, index_minus_one::*};
use druid::{widget::*, *};

#[derive(Clone, Data)]
pub struct Cell {
    pub value: Option<Num>,
    pub possibilities: IndexMinusOne<bool>,
    user_removed: IndexMinusOne<bool>,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            value: None,
            possibilities: IndexMinusOne::new(true),
            user_removed: IndexMinusOne::new(false),
        }
    }
}

impl Cell {
    fn possiblity_iter(&self) -> impl Iterator<Item = (bool, bool)> + '_ {
        self.possibilities
            .iter()
            .copied()
            .zip(self.user_removed.iter().copied())
    }

    // TODO override enumerate to handle +1 ?
    pub fn one_possibility(&self) -> Option<Num> {
        let mut ret = None;
        for (i, (p, ur)) in self.possiblity_iter().enumerate() {
            if p && !ur {
                if ret.is_none() {
                    ret = Some(i as Num + 1)
                } else {
                    return None;
                }
            }
        }
        ret
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
            || data.possiblity_iter().all(|(p, ur)| !p || ur)
        {
            Color::rgb(1.0, 0.6, 0.6)
        } else if data.value.is_none() && data.one_possibility().is_some() {
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
            ctx.submit_command(RECOMPUTE_SELECTOR, self.root);
            ctx.request_paint();
        }

        self.display.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Cell, env: &Env) {
        self.display.lifecycle(ctx, event, &data, env);

        match event {
            LifeCycle::WidgetAdded => ctx.register_for_focus(),

            LifeCycle::FocusChanged(focused) => {
                self.set_background_color(data, *focused);
                ctx.request_paint();
            }

            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Cell, data: &Cell, env: &Env) {
        self.set_background_color(data, ctx.has_focus());
        ctx.request_paint();
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
