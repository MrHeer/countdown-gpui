use std::sync::Arc;

use gpui::*;
use ui::{FluentBuilder, Tooltip};

type ConformHandler = dyn Fn(&u32, &mut WindowContext);
type Formatter = dyn Fn(&u32) -> AnyElement;

/// Used to control slider step util STEP pixels shift.
const STEP: f32 = 10.;

pub struct SliderNumber {
    id: ElementId,
    value: u32,
    min: u32,
    max: u32,
    conform_handler: Option<Arc<ConformHandler>>,
    formatter: Option<Arc<Formatter>>,
    start_value: u32,
    start_position: Option<Point<Pixels>>,
}

impl SliderNumber {
    pub fn new(id: impl Into<ElementId>, value: u32, min: u32, max: u32) -> Self {
        Self {
            id: id.into(),
            value,
            min,
            max,
            conform_handler: None,
            formatter: None,
            start_value: min,
            start_position: None,
        }
    }

    pub fn on_conform(mut self, handler: impl Fn(&u32, &mut WindowContext) + 'static) -> Self {
        self.conform_handler = Some(Arc::new(handler));
        self
    }

    pub fn formatter(mut self, formatter: impl Fn(&u32) -> AnyElement + 'static) -> Self {
        self.formatter = Some(Arc::new(formatter));
        self
    }

    fn on_drag_handler(this: &WeakView<Self>, cx: &mut WindowContext<'_>) -> View<EmptyView> {
        this.update(cx, |view, cx| {
            view.start_value = view.value;
            view.start_position = Some(cx.mouse_position());
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

        let position = cx.mouse_position();
        let start_y = this.start_position.unwrap().y;
        let y = position.y;
        let delta = (y - start_y) / STEP;
        let sign = delta.signum();
        let delta_value = delta.abs().floor().to_f64() as u32;
        if sign > 0. {
            this.value = (this.start_value + delta_value).clamp(this.min, this.max);
        } else {
            this.value =
                (this.start_value - delta_value.min(this.start_value)).clamp(this.min, this.max);
        }
    }
}

impl Render for SliderNumber {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let conform_handler = self.conform_handler.clone();
        let formatter = self.formatter.clone();
        let view = cx.view().downgrade();
        let value = self.value;
        div()
            .id(self.id.clone())
            .flex()
            .px_1()
            .tooltip(|cx| Tooltip::text("Drag to adjust, click to conform.", cx))
            .on_drag(self.id.clone(), move |_, cx| {
                Self::on_drag_handler(&view, cx)
            })
            .on_drag_move(cx.listener(Self::on_drag_move_handler))
            .when_some(conform_handler, |this, handler| {
                this.on_mouse_down(MouseButton::Left, |_, cx| cx.prevent_default())
                    .on_click(move |_, cx| {
                        cx.stop_propagation();
                        (handler)(&value, cx)
                    })
            })
            .when_some(formatter.clone(), |this, formatter| {
                this.child(formatter(&value))
            })
            .when(formatter.is_none(), |this| this.child(value.to_string()))
    }
}
