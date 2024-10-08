use anyhow::Context;
use anyhow::Result;
use ratatui::layout::*;
use ratatui::{backend::CrosstermBackend, *};
use std::env;
use std::io::stdout;

use segment::*;

// fn render_segments(segments: &mut [Box<dyn SegmentRenderer>]) -> Result<()> {
//     let backend = CrosstermBackend::new(stdout());
//     let options = TerminalOptions {
//         viewport: Viewport::Inline(segments.iter().map(|segment| segment.height()).sum()),
//     };
//     let mut terminal = Terminal::with_options(backend, options)?;

//     let constraints = segments
//         .iter()
//         .map(|segment| Constraint::Length(segment.height()))
//         .collect::<Vec<Constraint>>();

//     terminal.draw(|frame| {
//         let layout = Layout::default()
//             .direction(Direction::Vertical)
//             .constraints(constraints)
//             .split(frame.area());

//         for (segment, area) in segments.iter().zip(layout.iter()) {
//             segment.render(frame, *area).unwrap(); // Handle errors appropriately
//         }
//     })?;

//     Ok(())
// }

fn main() -> Result<()> {
    let backend = CrosstermBackend::new(stdout());

    let ip_info = ip::IpInfoBuilder::default().build()?;
    let heading_info = heading::HeadingSegmentInfoBuilder::default().build()?;

    let heading_renderer = heading::HeadingSegmentRenderer::new(heading_info);
    let heading_constraint = Constraint::Length(heading_renderer.height());

    let ip_renderer = ip::IpSegmentRenderer::new(ip_info);
    let ip_constraint = Constraint::Length(ip_renderer.height());
    println!("heading_renderer.height(): {}", heading_renderer.height());
    let options = TerminalOptions {
        viewport: Viewport::Inline(heading_renderer.height() + ip_renderer.height()),
    };
    let constraints = vec![heading_constraint, ip_constraint];
    let mut terminal = Terminal::with_options(backend, options)?;
    terminal.draw(|frame| {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(frame.area());

        heading_renderer.render(frame, layout[0]).unwrap();
        ip_renderer.render(frame, layout[1]).unwrap();
    })?;

    Ok(())
}

// fn main() -> Result<()> {
//     env::set_var("BASE_DIR", ".");
//     env::set_var("CONFIG_PATH", "./config.sh");

//     let mut segments: Vec<Box<dyn SegmentRenderer>> = vec![
//         // TODO: re-enable once rendering correctly
//         // Box::<heading::HeadingSegment>::default(),
//         Box::<quote::QuoteSegmentRenderer>::default(),
//         Box::new(<user::UserSegmentRenderer>::default()),
//         Box::new(<ip::IpSegmentRenderer>::default()),
//         Box::new(<os::OsSegmentRenderer>::default()),
//         Box::new(<uptime::UptimeSegmentRenderer>::default()),
//         Box::new(<load::LoadSegmentRenderer>::default()),
//         Box::new(<memory::MemorySegment>::default()),
//         Box::new(<updates::UpdatesSegmentRenderer>::default()),
//         Box::<disk::DiskSegmentRenderer>::default(),
//         // TODO: re-enable these after testing
//         // Box::<temperatures::TemperaturesSegment>::default(),
//         // Box::new(<docker::DockerSegment>::default())
//     ];

//     for segment in segments.iter_mut() {
//         segment
//             .prepare()
//             .with_context(|| format!("Failed to prepare segment: {:?}", segment))?;
//     }

//     render_segments(&mut segments)?;

//     Ok(())
// }
