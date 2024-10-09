use ansi_term::Colour::Blue;
use anyhow::Result;
use fmtsize::{Conventional, FmtSize};
use ratatui::{prelude::*, widgets::*};
use segment::*;
use sysinfo::System;

#[derive(Debug)]
pub struct MemorySegmentRenderer {
    info: MemoryInfo,
}

#[derive(Debug)]
pub struct MemoryInfo {
    used_memory: String,
    available_memory: String,
    total_memory: String,
}

impl Info for MemoryInfo {}

#[derive(Debug, Default)]
pub struct MemoryInfoBuilder {}

impl InfoBuilder<MemoryInfo> for MemoryInfoBuilder {
    fn build(&self) -> Result<MemoryInfo> {
        let mut sys = System::new_all();
        sys.refresh_all();

        // TODO: use consistent units, instead of letting Conventional decide
        let used_memory = sys.used_memory().fmt_size(Conventional).to_string();
        let available_memory = sys.available_memory().fmt_size(Conventional).to_string();
        let total_memory = sys.total_memory().fmt_size(Conventional).to_string();

        Ok(MemoryInfo {
            used_memory,
            available_memory,
            total_memory,
        })
    }
}

impl SegmentRenderer<MemoryInfo> for MemorySegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        // TODO: find way to re-use label and layout stuff
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new(Blue.bold().paint("RAM").to_string()),
            label_area,
        );

        frame.render_widget(
            Paragraph::new(format!(
                "{} used, {} available / {}",
                self.info.used_memory, self.info.available_memory, self.info.total_memory
            )),
            data_area,
        );

        Ok(())
    }
}

impl From<Box<MemoryInfo>> for MemorySegmentRenderer {
    fn from(info: Box<MemoryInfo>) -> Self {
        Self { info: *info }
    }
}
