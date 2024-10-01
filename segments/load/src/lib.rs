use ratatui::{prelude::*, widgets::*, TerminalOptions, Viewport};
use anyhow::Result;
use display::MotdSegment;
use std::io::stdout;
use sysinfo::System;

#[derive(Default, Debug)]
pub struct LoadSegment {
    info: Option<LoadInfo>,
}

#[derive(Debug)]
struct LoadInfo {
    loads: [f64; 3],
    cores: usize,
}

impl LoadInfo {
    fn new(loads: [f64; 3], cores: usize) -> Self {
        Self { loads, cores }
    }

    fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let loads = System::load_average();
        let cores = sys.physical_core_count().unwrap_or(1);

        Self::new([loads.one, loads.five, loads.fifteen], cores)
    }

    fn format(&self) -> Vec<Span> {
        let warning_threshold = self.cores as f64 * 0.9;
        let error_threshold = self.cores as f64 * 1.5;

        let colored_loads: Vec<Span> = self
            .loads
            .iter()
            .map(|&load| {
                let content = format!("{:.2}", load);
                if load < warning_threshold {
                    Span::styled(content, Style::default().fg(Color::Green))
                } else if load < error_threshold {
                    Span::styled(content, Style::default().fg(Color::Yellow))
                } else {
                    Span::styled(content, Style::default().fg(Color::Red))
                }
            })
            .collect();

        let mut result = vec![
            colored_loads[0].clone(),
            Span::raw(", "),
            colored_loads[1].clone(),
            Span::raw(", "),
            colored_loads[2].clone(),
            Span::raw(format!(" (across {} cores)", self.cores)),
        ];

        result
    }
}

impl MotdSegment for LoadSegment {
    fn prepare(&mut self) -> Result<()> {
        self.info = Some(LoadInfo::collect());
        Ok(())
    }

    fn render(&self) -> Result<()> {
        let backend = CrosstermBackend::new(stdout());
        let options = TerminalOptions {
            viewport: Viewport::Inline(1),
        };
        let mut terminal = Terminal::with_options(backend, options)?;

        terminal.draw(|frame| {
            let layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

            let [label_area, data_area] = layout.areas(frame.area());

            frame.render_widget(
                Paragraph::new("Load average").style(Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)),
                label_area,
            );

            if let Some(info) = &self.info {
                frame.render_widget(Paragraph::new(Line::from(info.format())), data_area);
            }
        })?;

        println!();

        Ok(())
    }
}
