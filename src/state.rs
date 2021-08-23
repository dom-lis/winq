use tui::layout::Rect;
use tui::buffer::Buffer;
use tui::widgets::Widget;
use tui::Terminal;
use tui::style::Style;

pub struct State {
    framed: bool,
    strings: Vec<String>,
}

pub struct Lines<'a> {
    strings: &'a [String]
}

impl <'a> Widget for Lines<'a> {
    fn render(self, rect: Rect, buffer: &mut Buffer) {
        for (y, s) in self.strings.iter().enumerate() {
            let y = y as u16 + rect.y;
            if y > rect.y + rect.height {
                break;
            }
            let x = rect.x;
            let w = (rect.width - rect.x).max(0) as usize;
            buffer.set_stringn(x, y, s, w, Style::default());
        }
    }
}

impl State {

    pub fn new() -> Self {
        State {
            framed: false,
            strings: Vec::new()
        }
    }

    pub fn frame(&mut self) {
        self.framed = true;
    }

    pub fn add_string(&mut self, s: String) {
        self.strings.push(s);
    }

    pub fn render<B: tui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> Result<(), std::io::Error> {
        if self.framed {
            self.framed = false;
            terminal.draw(|f| f.render_widget(Lines { strings: &self.strings }, f.size()))?;
            self.strings.clear();
        }
        Ok(())
    }

}
