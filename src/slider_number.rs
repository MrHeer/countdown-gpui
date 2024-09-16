use std::sync::Arc;

use gpui::*;
use ui::FluentBuilder;

type ChangeHandler = dyn Fn(&u32, &mut WindowContext);
type Formatter = dyn Fn(&u32) -> AnyElement;

/// Used to control slider step util STEP pixels shift.
const STEP: f32 = 10.;

pub struct SliderNumber {
    id: ElementId,
    value: u32,
    min: u32,
    max: u32,
    change_handler: Option<Arc<ChangeHandler>>,
    formatter: Option<Arc<Formatter>>,
    start_value: Option<u32>,
    start_position: Option<Point<Pixels>>,
}

impl SliderNumber {
    pub fn new(id: impl Into<ElementId>, value: u32, min: u32, max: u32) -> Self {
        Self {
            id: id.into(),
            value,
            min,
            max,
            change_handler: None,
            formatter: None,
            start_value: None,
            start_position: None,
        }
    }

    pub fn on_change(mut self, handler: impl Fn(&u32, &mut WindowContext) + 'static) -> Self {
        self.change_handler = Some(Arc::new(handler));
        self
    }

    pub fn formatter(mut self, formatter: impl Fn(&u32) -> AnyElement + 'static) -> Self {
        self.formatter = Some(Arc::new(formatter));
        self
    }

    fn on_drag_handler(slider: &WeakView<Self>, cx: &mut WindowContext<'_>) -> View<EmptyView> {
        slider
            .update(cx, |this, cx| {
                this.start_value = Some(this.value);
                this.start_position = Some(cx.mouse_position());
            })
            .ok();
        cx.new_view(|_| EmptyView)
    }

    fn on_drag_move_handler(
        this: &mut Self,
        event: &DragMoveEvent<ElementId>,
        cx: &mut ViewContext<'_, Self>,
    ) {
        if event.drag(cx) != &this.id {
            return;
        }

        let start_y = this.start_position.unwrap().y;
        let mut new_value = this.start_value.unwrap();
        let position = cx.mouse_position();
        let y = position.y;
        let delta = (y - start_y) / STEP;
        let sign = delta.signum();
        let delta_value = delta.abs().floor().to_f64() as u32;
        if sign > 0. {
            new_value = (new_value + delta_value).clamp(this.min, this.max);
        } else {
            new_value = (new_value - delta_value.min(new_value)).clamp(this.min, this.max);
        }
        if let Some(handler) = this.change_handler.clone() {
            handler(&new_value, cx);
        }
        this.value = new_value;
    }
}

impl Render for SliderNumber {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let formatter = self.formatter.clone();
        let view = cx.view().downgrade();
        let value = self.value;
        div()
            .id(self.id.clone())
            .hover(|style| style.cursor_ns_resize())
            .on_drag(self.id.clone(), move |_, cx| {
                Self::on_drag_handler(&view, cx)
            })
            .on_drag_move(cx.listener(Self::on_drag_move_handler))
            .when_some(formatter.clone(), |this, formatter| {
                this.child(formatter(&value))
            })
            .when(formatter.is_none(), |this| this.child(value.to_string()))
    }
}
