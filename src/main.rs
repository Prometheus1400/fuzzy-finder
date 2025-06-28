mod result_item;
mod score;

use crossterm::{
    cursor::{MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use score::score_items;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_buffer = String::new();
    io::stdin().read_to_string(&mut input_buffer).unwrap();
    let items: Vec<String> = input_buffer
        .lines()
        .map(|s| s.trim_start().to_string())
        .collect();

    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Show)?;
    let prompt = "> ";
    let mut input_query = String::new();
    let mut scored_items = score_items(&items, input_query.as_str());
    // Clear screen and redraw
    let limit = 50;
    let mut rerender = true;
    let mut top_item: String = String::new();
    loop {
        if rerender {
            top_item = String::new();
            execute!(
                stdout,
                Clear(ClearType::All),
                MoveTo(0, 0),
                SetForegroundColor(Color::Green),
                Print(prompt),
                ResetColor,
                Print(&input_query),
                SetForegroundColor(Color::White),
            )?;
            for i in 1..limit {
                if let Some(x) = scored_items.pop() {
                    if i == 1 {
                        top_item = x.content.clone();
                    }
                    execute!(
                        stdout,
                        MoveTo(0, i),
                        Print(format!("{}: ", i)),
                        Print(format!("{}\n", x.content))
                    )?;
                } else {
                    break;
                }
            }
            execute!(
                stdout,
                MoveTo(input_query.len() as u16 + prompt.len() as u16, 0),
            )?;
        }
        rerender = false;
        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    KeyCode::Esc => break,
                    KeyCode::Char(c) => {
                        input_query.push(c);
                        scored_items = score_items(&items, input_query.as_str());
                        rerender = true;
                    }
                    KeyCode::Backspace => {
                        input_query.pop();
                        scored_items = score_items(&items, input_query.as_str());
                        rerender = true;
                    }
                    KeyCode::Enter => {
                        execute!(
                            stdout,
                            Clear(ClearType::All),
                        )?;
                        disable_raw_mode()?;
                        println!("{}", top_item);
                        return Ok(());
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    Ok(())
}
