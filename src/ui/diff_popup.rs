use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::{app::App, git::diff::FilePatch};

#[derive(Clone, Default)]
pub struct DiffViewState {
    pub scroll: u16,
    pub file_index: usize,
}

impl DiffViewState {
    pub fn scroll(&mut self, delta: i16) {
        if delta < 0 {
            self.scroll = self.scroll.saturating_sub(-delta as u16);
        } else {
            self.scroll += delta as u16;
        }
    }

    pub fn scroll_file(&mut self, delta: isize, max: usize) {
        if delta < 0 {
            self.file_index = self.file_index.saturating_sub(-delta as usize);
        } else {
            if self.file_index + delta as usize >= max {
                self.file_index = max;
                return;
            }
            self.file_index += delta as usize;
        }
    }
}

pub struct DiffView {
    pub diff: Option<FilePatch>,
}

impl DiffView {
    pub fn new(app: &App) -> Self {
        Self {
            diff: app.inspect_patch.clone(),
        }
    }
}

impl StatefulWidget for DiffView {
    type State = DiffViewState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let lines: Vec<Line> = if let Some(diff) = self.diff.as_ref() {
            diff.lines.clone()
        } else {
            vec![Line::from("Loading diff...")]
        };
        let title = &self
            .diff
            .as_ref()
            .map(|patch| patch.path.to_string_lossy().into_owned())
            .unwrap_or_else(|| "Diff".to_string());

        let title = format!("{} - {}", title, state.file_index);

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title(title.as_str()))
            .scroll((state.scroll, 0));

        paragraph.render(area, buf);
    }
}

impl FilePatch {
    pub fn to_lines(raw_lines: &[String]) -> Vec<Line<'static>> {
        raw_lines
            .iter()
            .map(|text| {
                let style = match text.chars().next() {
                    Some('+') => Style::default().fg(Color::Green),
                    Some('-') => Style::default().fg(Color::Red),
                    _ => Style::default(),
                };
                Line::from(Span::styled(text.clone(), style))
            })
            .collect()
    }
}
