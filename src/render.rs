use std::io::{self, Stdout, Write};

use crossterm::{
    cursor,
    style::{self, Stylize},
    terminal, ExecutableCommand, QueueableCommand,
};

use crate::{hero::Hero, zombie::Zombie, Point2d};

pub fn render_screen(
    stdout: &mut Stdout,
    zombies: &[Zombie],
    hero: &Hero,
    screen_size: Point2d,
) -> io::Result<()> {
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
