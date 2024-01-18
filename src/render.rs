use crate::{bullets::Bullet, hero::Hero, types::Point2d, world::Screen, zombie::Zombie};
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{self, PrintStyledContent, StyledContent, Stylize},
    terminal::{self, size, ClearType},
    QueueableCommand,
};
use std::{
    fmt::Display,
    io::{self, Result, Stdout, Write},
};

// This could be used for rendering to a GUI, it is decoupled from the console
pub trait Renderer {
    fn clear(&mut self);
    fn screen_size(&self) -> Point2d;
    fn render(
        &mut self,
        zombies: &[Zombie],
        bullets: &[Bullet],
        hero: &Hero,
        current_screen: &Screen,
    ) -> Result<()>;
    fn cleanup(&mut self);
}

pub struct ConsoleRenderer {
    stdout: Stdout,
}
impl ConsoleRenderer {
    pub fn new() -> Self {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode().unwrap();
        execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide).unwrap();
        Self { stdout }
    }

    // In the refactored `draw_entity` function, you still have the flexibility to pass in a string literal (which implements `Display`)
    // and apply styling as you did before. But now, you could also use other types that implement the `Display` trait, not just string literals.
    // You pass the desired style as a parameter, which allows you to style any entity dynamically based on game state, entity type, or other conditions.
    // Display types include collections and custom types and enums etc, they can be turned into strings to print
    fn center_column_for_line_of_text(&self, message: &str) -> u16 {
        let screen_width = self.screen_size().y;
        screen_width / 2 - (message.chars().count() as u16) / 2
    }

    fn draw_centered_message(&mut self, message: &str) -> Result<()> {
        let start_row = self.screen_size().x / 2;
        let color = style::Color::Green;
        self.queue_styled_content(
            message.with(color),
            Point2d {
                x: start_row,
                y: self.center_column_for_line_of_text(message),
            },
        )
    }

    fn queue_styled_content<D: Display>(
        &mut self,
        content: StyledContent<D>,
        position: Point2d,
    ) -> Result<()> {
        self.stdout
            .queue(MoveTo(position.y, position.x))?
            .queue(PrintStyledContent(content))?;
        Ok(())
    }

    fn draw_entity<T: Display>(
        &mut self,
        entity_char: T,
        position: Point2d,
        color: style::Color,
    ) -> Result<()> {
        self.queue_styled_content(entity_char.to_string().with(color), position)
    }

    fn draw_start_menu(&mut self) -> Result<()> {
        self.draw_centered_message("Welcome to Zombie Attack, press s to start or q to quit")
    }

    fn draw_game_over(&mut self) -> Result<()> {
        self.draw_centered_message("You are dead! Game Over. s to start again or q to quit")
    }

    fn draw_in_game_stats(&mut self, lives: i32, zombies_left: usize) -> Result<()> {
        let message = format!("Lives: {lives} Zombies {zombies_left}");
        let start_row = 0;
        let start_column = self.center_column_for_line_of_text(&message);
        self.queue_styled_content(
            message.grey().reverse(),
            Point2d {
                x: start_row,
                y: start_column,
            },
        )
    }

    fn draw_rectangle(&mut self, width: u16, height: u16) -> Result<()> {
        // Drawing the top and bottom borders
        let border_char = "█".grey();
        for x in 0..width {
            self.queue_styled_content(border_char, Point2d { x: 0, y: x })?;
            self.queue_styled_content(
                border_char,
                Point2d {
                    x: height - 1,
                    y: x,
                },
            )?;
        }
        // Drawing the left and right borders (excluding corners to avoid over-drawing)
        for y in 1..height - 1 {
            self.queue_styled_content(border_char, Point2d { x: y, y: 0 })?;
            self.queue_styled_content(border_char, Point2d { x: y, y: width - 1 })?;
        }
        Ok(())
    }
}

impl Renderer for ConsoleRenderer {
    fn screen_size(&self) -> Point2d {
        let (number_cols, number_rows) = size().unwrap();
        Point2d {
            x: number_rows,
            y: number_cols,
        }
    }

    fn clear(&mut self) {
        execute!(self.stdout, terminal::Clear(ClearType::All)).unwrap();
    }

    fn render(
        &mut self,
        zombies: &[Zombie],
        bullets: &[Bullet],
        hero: &Hero,
        current_screen: &Screen,
    ) -> Result<()> {
        self.clear();

        match current_screen {
            Screen::StartMenu => self.draw_start_menu()?,
            Screen::GamePlay => {
                // draw_debug(&bullets[0], &mut self.stdout)?;
                self.draw_entity("h", hero.position, style::Color::Red)?;
                for bullet in bullets {
                    self.draw_entity("b", bullet.position, style::Color::Yellow)?;
                }
                for zombie in zombies {
                    self.draw_entity("z", zombie.position, style::Color::Green)?;
                }
                // Get screen size and use as a rectangle to draw the borders
                let (width, height) = size().unwrap();
                self.draw_rectangle(width, height)?;
                self.draw_in_game_stats(hero.lives, zombies.len())?;
            }
            Screen::GameOver => self.draw_game_over()?,
        }

        self.stdout.flush()?; // draw screen from queued buffer
        Ok(())
    }

    fn cleanup(&mut self) {
        // Revert any terminal changes
        execute!(self.stdout, cursor::Show, terminal::LeaveAlternateScreen).unwrap();
        terminal::disable_raw_mode().unwrap();
    }
}
