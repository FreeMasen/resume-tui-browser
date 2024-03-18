use ratatui::layout::Size;
use ratatui::style::Modifier;
use wasm_bindgen::prelude::*;

pub struct Terminal;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    fn writeToCanvas(s: String, x: u16, y: u16, bold: bool, italic: bool);
    #[wasm_bindgen]
    fn canvasSizeW() -> u16;
    #[wasm_bindgen]
    fn canvasSizeH() -> u16;
}

impl ratatui::backend::Backend for Terminal {
    fn draw<'a, I>(&mut self, content: I) -> std::io::Result<()>
    where
        I: Iterator<Item = (u16, u16, &'a ratatui::prelude::buffer::Cell)>,
    {
        for (x, y, cell) in content {
            let fg = match cell.fg {
                ratatui::style::Color::Green => ansi_rgb::green(),
                ratatui::style::Color::White => ansi_rgb::white(),
                ratatui::style::Color::LightGreen => ansi_rgb::yellow_green(),
                _ => ansi_rgb::black(),
            };

            let bg = match cell.bg {
                ratatui::style::Color::Green => ansi_rgb::green(),
                ratatui::style::Color::White => ansi_rgb::white(),
                ratatui::style::Color::LightGreen => ansi_rgb::yellow_green(),
                _ => ansi_rgb::black(),
            };
            use ansi_rgb::{Background, Foreground};
            writeToCanvas(
                format!("{}", cell.symbol().fg(fg).bg(bg)),
                x,
                y,
                cell.modifier.contains(Modifier::BOLD),
                cell.modifier.contains(Modifier::ITALIC),
            );
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn show_cursor(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn get_cursor(&mut self) -> std::io::Result<(u16, u16)> {
        Ok((0, 0))
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> std::io::Result<()> {
        Ok(())
    }

    fn clear(&mut self) -> std::io::Result<()> {
        Ok(())
    }

    fn size(&self) -> std::io::Result<ratatui::prelude::Rect> {
        let w = canvasSizeW();
        let h = canvasSizeH();
        Ok(ratatui::prelude::Rect::new(0, 0, w, h))
    }

    fn window_size(&mut self) -> std::io::Result<ratatui::prelude::backend::WindowSize> {
        let height = canvasSizeH();
        let width = canvasSizeW();
        Ok(ratatui::prelude::backend::WindowSize {
            columns_rows: Size { height, width },
            pixels: Size {
                height: 0,
                width: 0,
            },
        })
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
