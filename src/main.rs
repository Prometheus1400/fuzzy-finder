mod result_item;
mod score;

use crossterm::{
    cursor::{MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use result_item::ResultItem;
use score::score_items;
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input_buffer = String::new();
    io::stdin().read_to_string(&mut input_buffer).unwrap();
    let items: Vec<&str> = input_buffer.lines().map(|s| s.trim_start()).collect();

    enable_raw_mode()?;
    let mut stack: Vec<Vec<ResultItem>> = Vec::new();
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, Show)?;
    let prompt = "> ";
    let mut input_query = String::new();
    stack.push(score_items(&items, input_query.as_str()));
    // Clear screen and redraw
    let limit = 50;
    let mut rerender = true;
    let mut top_item: Option<String> = None;
    loop {
        if rerender {
            let items = stack.last().unwrap();
            top_item = None;
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
            for i in 0..limit {
                if i >= items.len() {
                    break;
                }
                if i == 0 {
                    top_item = Some(items[i].content.clone());
                }
                execute!(
                    stdout,
                    MoveTo(0, (i + 1).try_into().unwrap()),
                    Print(format!("{}: ", i)),
                    Print(format!("{}\n", items[i].content))
                )?;
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
                        let prev_items = stack
                            .last()
                            .unwrap()
                            .iter()
                            .map(|result_item| result_item.content.as_str())
                            .collect();
                        input_query.push(c);
                        stack.push(score_items(&prev_items, input_query.as_str()));
                        rerender = true;
                    }
                    KeyCode::Backspace => {
                        input_query.pop();
                        if stack.len() > 1 {
                            stack.pop();
                            rerender = true;
                        }
                    }
                    KeyCode::Enter => {
                        execute!(stdout, Clear(ClearType::All),)?;
                        disable_raw_mode()?;
                        if let Some(s) = top_item {
                            println!("{}", s);
                        }
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
