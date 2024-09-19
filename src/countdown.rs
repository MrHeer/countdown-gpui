use std::time::Duration;

use chrono::{DateTime, Days, Local, TimeDelta, Timelike};
use gpui::*;
use ui::v_flex;

use crate::{slider_number::SliderNumber, utils::format_duration};

pub struct Countdown {
    hour: u32,
    minute: u32,
    hour_slider: View<SliderNumber>,
    minute_slider: View<SliderNumber>,
}

fn build_slider(
    id: impl Into<ElementId>,
    value: u32,
    min: u32,
    max: u32,
    updater: impl Fn(&u32, &mut Countdown) + 'static,
    cx: &mut ViewContext<'_, Countdown>,
) -> View<SliderNumber> {
    let view = cx.view().downgrade();
    cx.new_view(|_| {
        SliderNumber::new(id, value, min, max)
            .on_change(move |value, cx| {
                view.update(cx, |this, _| {
                    updater(value, this);
                })
                .ok();
            })
            .formatter(|value| format!("{:02}", value).into_any_element())
    })
}

impl Countdown {
    pub fn new(hour: u32, minute: u32, cx: &mut ViewContext<'_, Self>) -> Self {
        let hour_slider = build_slider("hour", hour, 0, 23, |value, this| this.hour = *value, cx);
        let minute_slider = build_slider(
            "minute",
            minute,
            0,
            59,
            |value, this| this.minute = *value,
            cx,
        );
        Self {
            hour,
            minute,
            hour_slider,
            minute_slider,
        }
    }

    fn get_time(&self, now: &DateTime<Local>) -> DateTime<Local> {
        now.with_hour(self.hour)
            .unwrap()
            .with_minute(self.minute)
            .unwrap()
    }

    fn get_duration(&self) -> TimeDelta {
        let now = Local::now();
        let mut time = self.get_time(&now);
        if time < now {
            time = time + Days::new(1);
        }
        time - now
    }

    fn render_after_secs(&self, secs: u32, cx: &mut ViewContext<Self>) {
        cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .timer(Duration::from_secs(secs as u64))
                .await;
            this.update(&mut cx, |_, cx| {
                cx.notify();
            })
            .ok()
        })
        .detach();
    }
}

impl Render for Countdown {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.render_after_secs(60, cx);
        v_flex()
            .justify_center()
            .items_center()
            .gap_5()
            .size_full()
            .bg(rgba(0xffffff99))
            .font_family("MonaspiceRn Nerd Font Mono")
            .child("After")
            .child(
                div()
                    .flex()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .child(format_duration(&self.get_duration())),
            )
            .child("To")
            .child(
                div()
                    .flex()
                    .text_2xl()
                    .child(self.hour_slider.clone())
                    .child(":")
                    .child(self.minute_slider.clone()),
            )
    }
}
