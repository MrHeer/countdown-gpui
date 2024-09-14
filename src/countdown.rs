use std::time::Duration;

use chrono::{DateTime, Days, Local, TimeDelta, Timelike};
use gpui::*;
use ui::v_flex;

use crate::utils::{format_duration, format_hm};

pub struct Countdown {
    hour: u32,
    minute: u32,
}

impl Countdown {
    pub fn new(hour: u32, minute: u32) -> Self {
        Self { hour, minute }
    }

    fn format_time(&self) -> String {
        format_hm(self.hour, self.minute)
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
            this.update(&mut cx, |_this, cx| {
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
            .child("After")
            .child(
                div()
                    .flex()
                    .text_3xl()
                    .font_weight(FontWeight::BOLD)
                    .child(format_duration(&self.get_duration())),
            )
            .child("To")
            .child(div().flex().text_2xl().child(self.format_time()))
    }
}
