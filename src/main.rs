// The ascii images come from this mod
mod image_vector;
use image_vector::get_image_vector;

// Small ascii images come from this other file
mod small_image_vector;
use small_image_vector::get_small_image_vector;

// The mga small ascii images
mod mega_small_image_vector;
use mega_small_image_vector::get_mega_small_image_vector;

// The library to generate random number
use rand::Rng;

// crossterm is a library to handle terminal things
use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

// ratatui is the main library, and it's for TUIS (Terminal User InterfaceS)
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::{CrosstermBackend, Styled, Terminal},
    style::Stylize,
    text::{Span, Text},
    widgets::{Block, Borders, Paragraph},
};
use term_size::dimensions;

// Whith this 2 packages, we handle the standar outputs and Results from the terminal
use std::io::{stdout, Result};

fn main() -> Result<()> {
    // First settings to start the terminal and output things
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // Start using the terminal and clear all the current info apearing
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Using our image_vector file, we set the 1 picture to be the 1st element of the pictures
    // vector
    let mut current_picture = get_image_vector()[0];

    // Initialize the small vector image
    let mut small_current_picture = get_small_image_vector()[0];

    // Initialize the mega small image vector image
    let mut mega_small_current_picture = get_mega_small_image_vector()[0];

    // We initialize our random number generator and random_number
    let mut rng = rand::thread_rng();
    let mut random_number;

    // TODO main loop (All the terminal things happen here)
    loop {
        // Draw the main UI, and we a closure, where the main "variable" is frame
        terminal.draw(|frame| {
            // We set the picture that we will be showing to be a Paragraph containing the
            // current_picture (Managed by the pictures vector)
            let picture = Paragraph::new(current_picture);

            let small_picture = Paragraph::new(small_current_picture);

            let mega_small_picture = Paragraph::new(mega_small_current_picture);

            // This is the main thing that we renderize, and it's the picture, inside a block with
            // borders and centered
            let banner_widget = picture
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);

            let small_banner_widget = small_picture
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);

            let mega_small_banner_widget = mega_small_picture
                .block(Block::default().borders(Borders::ALL))
                .alignment(Alignment::Center);

            // And then we renderize the picture in the block
            // render_widget takes 2 parameters, the 1st one is what we want to show (waifu img)
            // and the 2nd is the size/space that the picture will take place in
            // in this case, we're calling our function centered_rect to center our figures, inside
            // a frame.size of our screen, and height/width of size 60 each one
            if let Some((width, height)) = dimensions() {
                if height >= 1 && height < 21 {
                    frame.render_widget(
                        mega_small_banner_widget,
                        centered_rect(frame.size(), 60, 100),
                    )
                } else if height >= 21 && height <= 49 {
                    frame.render_widget(small_banner_widget, centered_rect(frame.size(), 60, 100));
                }

                frame.render_widget(banner_widget, centered_rect(frame.size(), 60, 100));
            }
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                match key.code {
                    // In case you press q, the program will finish
                    KeyCode::Char('q') => break,
                    // In case you press c, the wallpaper will change randomly
                    // using our pictures vector, and the random number generator
                    KeyCode::Char('c') => {
                        random_number = rng.gen_range(0..get_image_vector().len());

                        if let Some((width, height)) = dimensions() {
                            if height >= 1 && height < 21 {
                                mega_small_current_picture =
                                    get_mega_small_image_vector()[random_number]
                            } else if height >= 21 && height < 49 {
                                small_current_picture = get_small_image_vector()[random_number];
                            }

                            current_picture = get_image_vector()[random_number];
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    disable_raw_mode()?;
    Ok(())
}

// Function to center the img's
fn centered_rect(r: Rect, percent_x: u16, percent_y: u16) -> Rect {
    // First, we center the screen in a vertically way
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // And then, in an Horizontally way
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
