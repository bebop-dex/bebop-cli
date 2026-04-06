use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};

const Y: u8 = 0; // yellow
const B: u8 = 1; // black (glasses)
const T: u8 = 2; // transparent

const ICON_W: usize = 18;
const ICON_H: usize = 18;

#[rustfmt::skip]
const PIXELS: [[u8; ICON_W]; ICON_H] = [
    [T,T,T,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,T,T,T],
    [T,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,T],
    [T,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,T],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [Y,Y,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,Y],
    [Y,B,B,B,B,B,B,B,B,B,B,B,B,B,B,B,Y,Y],
    [Y,B,B,B,B,B,B,Y,B,B,B,B,B,B,Y,Y,Y,Y],
    [Y,B,B,B,B,B,B,Y,B,B,B,B,B,B,Y,Y,Y,Y],
    [Y,Y,B,B,B,B,Y,Y,Y,B,B,B,B,Y,Y,Y,Y,Y],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y],
    [T,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,T],
    [T,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,T],
    [T,T,T,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,Y,T,T,T],
];

fn pixel_color(p: u8) -> Option<Color> {
    match p {
        Y => Some(Color::Rgb(212, 255, 0)),
        B => Some(Color::Rgb(23, 23, 23)),
        _ => None,
    }
}

pub fn icon_lines() -> Vec<Line<'static>> {
    let mut lines = Vec::with_capacity(ICON_H / 2);

    for pair in 0..(ICON_H / 2) {
        let top_row = &PIXELS[pair * 2];
        let bot_row = &PIXELS[pair * 2 + 1];
        let mut spans = Vec::with_capacity(ICON_W);

        for x in 0..ICON_W {
            let top = pixel_color(top_row[x]);
            let bot = pixel_color(bot_row[x]);

            let (ch, style) = match (top, bot) {
                (None, None) => (' ', Style::default()),
                (Some(tc), None) => ('\u{2580}', Style::new().fg(tc)),
                (None, Some(bc)) => ('\u{2584}', Style::new().fg(bc)),
                (Some(tc), Some(bc)) if tc == bc => ('\u{2588}', Style::new().fg(tc)),
                (Some(tc), Some(bc)) => ('\u{2580}', Style::new().fg(tc).bg(bc)),
            };

            spans.push(Span::styled(String::from(ch), style));
        }

        lines.push(Line::from(spans));
    }

    lines
}
