use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Clear, Paragraph};

use super::centered_rect;
use crate::tui::state::theme;

const SECTIONS: &[(&str, &[(&str, &str)])] = &[
    (
        "Global",
        &[
            ("\u{2190}/\u{2192} or Tab/Shift-Tab", "Switch tabs"),
            ("Shift+C", "Global chain selector"),
            ("?", "Toggle this help"),
            ("q", "Quit"),
        ],
    ),
    (
        "Tokens",
        &[
            ("\u{2191}/\u{2193}", "Navigate token list"),
            ("/", "Search / filter"),
            ("c", "Change chain"),
            ("s", "Use as sell token \u{2192} Quote"),
            ("b", "Use as buy token \u{2192} Quote"),
        ],
    ),
    (
        "Quote",
        &[
            ("n", "New quote"),
            ("\u{2191}/\u{2193}", "Navigate quote table"),
            ("d", "Delete selected quote"),
        ],
    ),
    (
        "Quote Form",
        &[
            ("\u{2191}/\u{2193}", "Navigate fields"),
            ("Enter", "Activate / select field"),
            ("c", "Open chain picker"),
            ("Esc", "Close form"),
        ],
    ),
    (
        "Config",
        &[
            ("\u{2191}/\u{2193} or Tab", "Navigate fields"),
            ("Enter", "Edit / activate field"),
            ("Esc", "Cancel editing"),
        ],
    ),
];

pub fn render(frame: &mut Frame, area: Rect, scroll_offset: usize) {
    // Build content lines
    let mut lines: Vec<Line> = Vec::new();

    for (i, (section, bindings)) in SECTIONS.iter().enumerate() {
        if i > 0 {
            lines.push(Line::raw(""));
        }
        lines.push(Line::from(Span::styled(
            format!("  {section}"),
            theme::HELP_SECTION,
        )));

        for (key, desc) in *bindings {
            lines.push(Line::from(vec![
                Span::raw("    "),
                Span::styled(format!("{key:<28}"), theme::HELP_KEY),
                Span::styled(*desc, theme::HELP_DESC),
            ]));
        }
    }

    let total_lines = lines.len();

    // Popup size
    let popup_width = 60u16.min(area.width.saturating_sub(4));
    let popup_height = 24u16.min(area.height.saturating_sub(2)).max(8);
    let popup_area = centered_rect(popup_width, popup_height, area);

    frame.render_widget(Clear, popup_area);

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(theme::HELP_BORDER)
        .title(Span::styled(" Keybindings ", theme::HELP_TITLE));

    let inner = block.inner(popup_area);
    frame.render_widget(block, popup_area);

    let viewport_height = inner.height as usize;
    let max_scroll = total_lines.saturating_sub(viewport_height);
    let scroll = scroll_offset.min(max_scroll);

    let visible: Vec<Line> = lines.into_iter().skip(scroll).take(viewport_height).collect();

    // Scroll indicator
    let mut display_lines = visible;
    if max_scroll > 0 && display_lines.len() == viewport_height {
        let last = display_lines.len() - 1;
        display_lines[last] = Line::from(Span::styled(
            "  \u{2191}\u{2193} scroll   Esc/? close",
            theme::HELP_SECTION,
        ));
    }

    frame.render_widget(Paragraph::new(display_lines), inner);
}
