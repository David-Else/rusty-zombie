use crate::{hero::Hero, types::Point2d, zombie::Zombie};
use crossterm::{
    cursor::{self, Hide, Show},
    style::{self, Stylize},
    terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Result, Stdout, Write};

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

pub fn setup_terminal() -> Result<(io::Stdout, Point2d)> {
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;
    let screensize = {
        let (number_cols, number_rows) = size()?;
        Point2d {
            x: number_rows as usize,
            y: number_cols as usize,
        }
    };
    Ok((stdout, screensize))
}

pub fn cleanup_terminal(mut stdout: io::Stdout) -> Result<()> {
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
