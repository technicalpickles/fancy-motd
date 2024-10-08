use anyhow::Result;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::*,
    style::{Color, Modifier, Style},
    widgets::Paragraph,
};

use std::fmt::Debug;

pub trait MotdSegment: Debug {
    fn prepare(&mut self) -> Result<()>;
    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()>;
    fn height(&self) -> u16;
}

#[derive(Debug)]
pub struct Single {
    content: String,
}

impl Single {
    pub fn new(content: &str) -> Self {
        Self {
            content: content.to_string(),
        }
    }
}

impl MotdSegment for Single {
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }
    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let paragraph = Paragraph::new(self.content.clone()).style(Style::default());
        frame.render_widget(paragraph, area);
        Ok(())
    }
    fn height(&self) -> u16 {
        self.content.lines().count() as u16
    }
}

#[derive(Debug)]
pub struct LabelWithContent {
    label: String,
    content: String,
}

impl LabelWithContent {
    pub fn new(label: &str, content: &str) -> Self {
        Self {
            label: label.to_string(),
            content: content.to_string(),
        }
    }
}

impl MotdSegment for LabelWithContent {
    fn prepare(&mut self) -> Result<()> {
        Ok(())
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Length(16), Constraint::Min(0)].as_ref())
            .split(area);

        let label = Paragraph::new(format!("{}:", self.label)).style(
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        );
        frame.render_widget(label, chunks[0]);

        let content = Paragraph::new(self.content.clone());
        frame.render_widget(content, chunks[1]);
        Ok(())
    }
    fn height(&self) -> u16 {
        self.content.lines().count() as u16 + 1
    }
}
