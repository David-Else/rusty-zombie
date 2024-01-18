use crate::{bullets::Bullet, hero::Hero, types::Point2d, world::Screen, zombie::Zombie};
use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::{self, PrintStyledContent, Stylize},
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
    fn draw_entity<T: Display>(
        &mut self,
        entity: &T,
        position: &Point2d,
        color: style::Color,
    ) -> Result<()> {
        self.stdout
            .queue(MoveTo(position.y, position.x))?
            .queue(PrintStyledContent(entity.to_string().with(color)))?;
        Ok(())
    }

    fn draw_start_menu(&mut self) -> Result<()> {
        let message = "Welcome to Zombie Attack, press s to start or q to quit";
        let start_column = self.screen_size().y / 2 - (message.chars().count() as u16) / 2;
        let start_row = self.screen_size().x / 2;
        self.stdout
            .queue(MoveTo(start_column, start_row))?
            .queue(PrintStyledContent(message.green()))?;
        Ok(())
    }

    fn draw_game_over(&mut self) -> Result<()> {
        let message = "You are dead! Game Over. s to start again or q to quit";
        let start_column = self.screen_size().y / 2 - (message.chars().count() as u16) / 2;
        let start_row = self.screen_size().x / 2;
        self.stdout
            .queue(MoveTo(start_column, start_row))?
            .queue(PrintStyledContent(message.red()))?;
        Ok(())
    }

    fn draw_in_game_stats(&mut self, lives: &i32, zombies_left: &usize) -> Result<()> {
        let message = format!("Lives: {lives} Zombies {zombies_left}");
        let start_column = self.screen_size().y / 2 - (message.chars().count() as u16) / 2;
        let start_row = 0;

        self.stdout
            .queue(MoveTo(start_column, start_row))?
            .queue(PrintStyledContent(message.grey().reverse()))?;
        // Is this needed, will other text inherit grey reversed?
        // .queue(SetAttribute(Attribute::Reset))? // Reset all attributes
        // .queue(SetForegroundColor(Color::Reset))?; // Reset the foreground color

        Ok(())
    }

    fn draw_rectangle(&mut self, width: u16, height: u16) -> Result<()> {
        // Drawing the top and bottom borders
        for x in 0..width {
            self.stdout
                .queue(MoveTo(x, 0))?
                .queue(PrintStyledContent("█".grey()))?
                .queue(MoveTo(x, height - 1))?
                .queue(PrintStyledContent("█".grey()))?;
        }

        // Drawing the left and right borders (excluding corners to avoid over-drawing)
        for y in 1..height - 1 {
            self.stdout
                .queue(MoveTo(0, y))?
                .queue(PrintStyledContent("█".grey()))?
                .queue(MoveTo(width - 1, y))?
                .queue(PrintStyledContent("█".grey()))?;
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
                self.draw_entity(&"h", &hero.position, style::Color::Red)?;
                for bullet in bullets {
                    self.draw_entity(&"b", &bullet.position, style::Color::Yellow)?;
                }
                for zombie in zombies {
                    self.draw_entity(&"z", &zombie.position, style::Color::Green)?;
                }
                // Get screen size and use as a rectangle to draw the borders
                let (width, height) = size().unwrap();
                self.draw_rectangle(width, height)?;
                self.draw_in_game_stats(&hero.lives, &zombies.len())?;
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
