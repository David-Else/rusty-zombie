use crate::{bullets::Bullet, hero::Hero, types::Point2d, world::Screen, zombie::Zombie};
use crossterm::{
    cursor::{self, Hide, Show},
    style::{self, Print, Stylize},
    terminal::{self, size, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use std::io::{self, Result, Write};

pub fn render_screen(
    zombies: &[Zombie],
    bullets: &[Bullet],
    hero: &Hero,
    screen_size: &Point2d,
    current_screen: &Screen,
) -> Result<()> {
    let mut stdout = io::stdout(); // Get the standard output stream
    stdout.execute(terminal::Clear(terminal::ClearType::All))?;

    match current_screen {
        Screen::StartMenu => draw_start_menu(screen_size, &mut stdout)?,
        Screen::GamePlay => {
            draw_debug(&bullets[0], &mut stdout)?;
            draw_hero(hero, &mut stdout)?;
            draw_bullet(bullets, &mut stdout)?;
            draw_zombie(zombies, &mut stdout)?;
            draw_borders(screen_size, &mut stdout)?;
        }
    }

    stdout.flush()?; // draw screen from queued buffer
    Ok(())
}

fn draw_start_menu(screen_size: &Point2d, stdout: &mut io::Stdout) -> Result<()> {
    let message = "Welcome to Zombie Attack, press s to start or q to quit";
    let start_column = (screen_size.y as u16) / 2 - (message.chars().count() as u16) / 2;
    let start_row = (screen_size.x as u16) / 2;
    stdout
        .queue(cursor::MoveTo(start_column, start_row))?
        .queue(style::PrintStyledContent(message.green()))?;
    Ok(())
}

fn draw_hero(hero: &Hero, stdout: &mut io::Stdout) -> Result<()> {
    stdout
        .queue(cursor::MoveTo(
            hero.position.y as u16,
            hero.position.x as u16,
        ))?
        .queue(style::PrintStyledContent("h".red()))?;
    Ok(())
}

fn draw_debug(object: &Bullet, stdout: &mut io::Stdout) -> Result<()> {
    stdout
        .queue(cursor::MoveTo(10, 1))?
        .queue(Print(format!("{:?}", object)))?;
    Ok(())
}

fn draw_bullet(bullets: &[Bullet], stdout: &mut io::Stdout) -> Result<()> {
    for bullet in bullets.iter() {
        stdout
            .queue(cursor::MoveTo(
                bullet.position.y as u16,
                bullet.position.x as u16,
            ))?
            .queue(style::PrintStyledContent("b".yellow()))?;
    }
    Ok(())
}

fn draw_zombie(zombies: &[Zombie], stdout: &mut io::Stdout) -> Result<()> {
    for zombie in zombies.iter() {
        stdout
            .queue(cursor::MoveTo(
                zombie.position.y as u16,
                zombie.position.x as u16,
            ))?
            .queue(style::PrintStyledContent("z".green()))?;
    }
    Ok(())
}

fn draw_borders(screen_size: &Point2d, stdout: &mut io::Stdout) -> Result<()> {
    for y in 0..screen_size.y {
        for x in 0..screen_size.x {
            if (y == 0 || y == screen_size.y - 1) || (x == 0 || x == screen_size.x - 1) {
                stdout
                    .queue(cursor::MoveTo(y as u16, x as u16))?
                    .queue(style::PrintStyledContent("█".grey()))?;
            }
        }
    }
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
        Ok(Self {
            stdout,
            screen_size: Point2d {
                x: number_rows as usize,
                y: number_cols as usize,
            },
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
