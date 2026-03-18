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
        if let Some(result) = self.scroll.checked_sub(delta as u16) {
            self.scroll = result;
        } else {
            self.scroll = 0;
        }
    }

    pub fn scroll_file(&mut self, delta: isize) {
        if let Some(result) = self.file_index.checked_sub(delta as usize) {
            self.file_index = result;
        } else {
            self.file_index = 0;
        }
    }
}

pub struct DiffView {
    pub diff: Option<FilePatch>,
}

impl DiffView {
    pub fn new(app: &mut App) -> Self {
        let result = app.inspect_file(0).is_ok();
        let diff = if result {
            app.inspect_patch.clone()
        } else {
            None
        };
        Self { diff: diff }
    }
}

impl<'a> StatefulWidget for DiffView {
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
