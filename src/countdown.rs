use chrono::{DateTime, Days, Duration, Local, NaiveTime};
use gpui::*;
use ui::v_flex;

use crate::utils::{format_datetime, format_duration};

pub struct Countdown {
    time: DateTime<Local>,
}

impl Countdown {
    fn get_duration(&self) -> Duration {
        let now = Local::now();
        let mut time = self.time;
        if time < now {
            time = time + Days::new(1);
        }
        time - now
    }

    pub fn from_hm(hour: u32, min: u32) -> Self {
        let time = Local::now()
            .with_time(NaiveTime::from_hms_opt(hour, min, 0).unwrap())
            .unwrap();
        Self { time }
    }
}

impl Render for Countdown {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
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
            .child(div().flex().text_2xl().child(format_datetime(&self.time)))
    }
}
