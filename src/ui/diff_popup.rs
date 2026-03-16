use std::path::PathBuf;

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, StatefulWidget, Widget},
};

use crate::git::diff::FilePatch;

#[derive(Clone)]
pub struct DiffViewState {
    pub scroll: u16,
    pub file_index: usize,
}

pub struct DiffView<'a> {
    pub diff: Option<&'a FilePatch>,
}

impl<'a> StatefulWidget for DiffView<'a> {
    type State = DiffViewState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let lines: Vec<Line> = if let Some(diff) = self.diff {
            diff.lines.clone()
        } else {
            vec![Line::from("Loading diff...")]
        };

        let paragraph = Paragraph::new(lines)
            .block(Block::default().borders(Borders::ALL).title("Diff"))
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
