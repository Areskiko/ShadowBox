use druid::{Widget, widget::{Painter, Flex}};
use druid::{RenderContext};

use crate::logic::appdata::AppData;


pub fn make_ui() -> impl Widget<AppData> {
    let top_left = Painter::new(|ctx, data: &AppData, _env| {
        let rect = ctx.size().to_rounded_rect(5.0);
        ctx.fill(rect, &data.top_left);
    });

    let top_right = Painter::new(|ctx, data: &AppData, _env| {
        let rect = ctx.size().to_rounded_rect(5.0);
        ctx.fill(rect, &data.top_right);
    });

    let bottom = Painter::new(|ctx, data: &AppData, _env| {
        let rect = ctx.size().to_rounded_rect(5.0);
        ctx.fill(rect, &data.bottom);
    });

    Flex::column()
        .with_flex_child(
            Flex::row()
                .with_flex_child(top_left, 7.0)
                .with_flex_spacer(0.1)
                .with_flex_child(top_right, 7.0),
            5.0,
        )
        .with_flex_spacer(0.1)
        .with_flex_child(bottom, 5.0)
}