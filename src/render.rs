use crate::{hero::Hero, zombie::Zombie, Point2d};
use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal::{self, size},
    ExecutableCommand, QueueableCommand,
};
use std::io::{Result, Stdout, Write};

pub fn render_screen(
    stdout: &mut Stdout,
    zombies: &[Zombie],
    hero: &Hero,
    screen_size: Point2d,
) -> Result<()> {
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

pub fn print_middle_screen(stdout: &mut Stdout, text: &str) -> Result<()> {
    let (cols, rows) = size()?; // Get the number of columns and rows of the terminal window

    // Move cursor to the calculated position and print the text
    stdout
        .queue(cursor::MoveTo(cols / 2, rows / 2))?
        .queue(crossterm::style::Print(text))? // Print the text
        .flush()?; // Flush the stdout to immediately output the text

    Ok(())
}
