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
    fn background_color(&self, focused: bool) -> Color {
        if focused {
            Color::rgb(0.6, 0.8, 1.0)
        } else if (self.value.is_some() && !self.possibilities[self.value.unwrap()])
            || self.possibilities.iter().all(|p| !p)
        {
            Color::rgb(1.0, 0.6, 0.6)
        } else if self.value.is_none() && self.possibilities.iter().filter(|&&x| x).count() == 1 {
            Color::rgb(0.7, 1.0, 0.7)
        } else {
            Color::WHITE
        }
    }
}

pub struct GridSpace {
    root: WidgetId,
    display: Container<Cell>,
}

impl GridSpace {
    pub fn new(root: WidgetId) -> Self {
        Self {
            root,
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
                        // TODO add better formatting to distinguish cases
                        if c.possibilities[num] && !c.user_removed[num] {
                            radix_string(num)
                        } else {
                            String::new()
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
}

impl Widget<Cell> for GridSpace {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut Cell, env: &Env) {
        // TODO handle arrow keys and tab
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
                            data.user_removed[num] = !data.user_removed[num];
                        } else {
                            new_val = press;
                        }
                    }
                }

                // TODO add others?
                KbKey::Backspace | KbKey::Delete => {
                    new_val = None;
                }

                _ => {}
            },
            Event::MouseDown(_) => ctx.request_focus(),
            _ => {}
        };

        if new_val != data.value {
            data.value = new_val;
            ctx.submit_command(RECOMPUTE_SELECTOR.with(()), self.root);
            ctx.request_paint();
        }

        self.display.event(ctx, event, data, env);
    }

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &Cell, env: &Env) {
        self.display.lifecycle(ctx, event, &data, env);

        match event {
            LifeCycle::WidgetAdded => ctx.register_for_focus(),

            LifeCycle::FocusChanged(focused) => {
                self.display.set_background(data.background_color(*focused));
                ctx.request_paint();
            }

            _ => {}
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &Cell, data: &Cell, env: &Env) {
        self.display
            .set_background(data.background_color(ctx.has_focus()));
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
