use crate::{hero::Hero, types::Point2d, zombie::Zombie};
use crossterm::{
    cursor::{self, Hide, Show},
    style::{self, Stylize},
    terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Result, Write};

pub fn render_screen(zombies: &[Zombie], hero: &Hero, screen_size: &Point2d) -> Result<()> {
    let mut stdout = io::stdout(); // Get the standard output stream
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    // draw borders
    for y in 0..screen_size.y {
        for x in 0..screen_size.x {
            if (y == 0 || y == screen_size.y - 1) || (x == 0 || x == screen_size.x - 1) {
                stdout
                    .queue(cursor::MoveTo(y as u16, x as u16))?
                    .queue(style::PrintStyledContent("█".grey()))?;
            }
        }
    }

    // draw zombies
    for zombie in zombies.iter() {
        stdout
            .queue(cursor::MoveTo(
                zombie.position.y as u16,
                zombie.position.x as u16,
            ))?
            .queue(style::PrintStyledContent("z".green()))?;
    }

    // draw hero
    stdout
        .queue(cursor::MoveTo(
            hero.position.y as u16,
            hero.position.x as u16,
        ))?
        .queue(style::PrintStyledContent("h".red()))?;

    // draw screen from queued buffer
    stdout.flush()?;
    Ok(())
}

pub struct Terminal {
    stdout: std::io::Stdout,
    pub screen_size: Point2d,
}

impl Terminal {
    // Constructor method to create a new Terminal instance and set up the terminal
    pub fn new() -> io::Result<Self> {
        let mut stdout = io::stdout();
        terminal::enable_raw_mode()?;
        stdout.execute(EnterAlternateScreen)?;
        stdout.execute(Hide)?;

        let (number_cols, number_rows) = size()?;
        let screen_size = Point2d {
            x: number_rows as usize,
            y: number_cols as usize,
        };

        Ok(Self {
            stdout,
            screen_size,
        })
    }

    // Cleanup method to restore the terminal to its previous state
    pub fn cleanup(&mut self) -> io::Result<()> {
        self.stdout.execute(Show)?;
        self.stdout.execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}
