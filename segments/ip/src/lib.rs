use anyhow::Result;
use local_ip_address::local_ip;
use ratatui::{prelude::*, widgets::*};
use segment::*;

#[derive(Debug)]
pub struct IpInfo {
    ip_address: String,
}

impl Info for IpInfo {}

#[derive(Debug, Default)]
pub struct IpInfoBuilder {}

impl InfoBuilder<IpInfo> for IpInfoBuilder {
    fn build(&self) -> Result<IpInfo> {
        let ip = local_ip()?;
        let ip_address = ip.to_string();
        Ok(IpInfo { ip_address })
    }
}

#[derive(Debug)]
pub struct IpSegmentRenderer {
    info: IpInfo,
}

impl SegmentRenderer<IpInfo> for IpSegmentRenderer {
    fn height(&self) -> u16 {
        1
    }

    fn render(&self, frame: &mut Frame, area: Rect) -> Result<()> {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Length(16), Constraint::Fill(1)]);

        let [label_area, data_area] = layout.areas(area);

        frame.render_widget(
            Paragraph::new("IP address").fg(Color::Blue).bold(),
            label_area,
        );

        frame.render_widget(Paragraph::new(self.info.ip_address.clone()), data_area);

        Ok(())
    }
}

impl From<Box<IpInfo>> for IpSegmentRenderer {
    fn from(info: Box<IpInfo>) -> Self {
        Self { info: *info }
    }
}
