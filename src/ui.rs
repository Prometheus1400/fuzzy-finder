use std::{
    io::{self, Stdout, Write},
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    time::Duration,
};

use crossterm::{
    cursor::{MoveTo, Show},
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{Clear, ClearType, EnterAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use tokio::sync::{
    RwLock,
    mpsc::{self, Receiver, Sender},
};

use crate::result_item::ResultItem;
use crate::score::score_items;

pub struct UI {
    inputs: Vec<String>, // the initial inputs from stdin
    // receiver: Option<Receiver<Vec<ResultItem>>>, // collector for scoring results, handles pushing them into 'matches'
    // sender: Option<Sender<Vec<ResultItem>>>,          // should be passed into the scoring tasks for sending their results back
    query: String,
    matches: Arc<RwLock<Vec<ResultItem>>>,
    stdout: Stdout,
    prompt: String,
    match_display_limit: usize,
    refresh_interval_ms: u64,
    current_task_id: Arc<AtomicU64>,
    top_item: Arc<RwLock<Option<String>>>,
}

impl UI {
    pub fn new(inputs: Vec<String>) -> Self {
        Self {
            inputs,
            // receiver,
            // sender,
            query: String::new(),
            matches: Arc::new(RwLock::new(Vec::new())),
            stdout: io::stdout(),
            prompt: "> ".to_string(),
            match_display_limit: 25,
            refresh_interval_ms: 100,
            current_task_id: Arc::new(AtomicU64::new(0)),
            top_item: Arc::new(RwLock::new(None)),
        }
    }

    /// Enters the UI input loop, this blocks the calling thread
    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let (sender, receiver) = mpsc::channel::<Vec<ResultItem>>(1);
        enable_raw_mode()?;
        execute!(self.stdout, EnterAlternateScreen, Show)?;
        self.stdout.flush()?;
        self.rescore_items(&sender).await;
        // let mut ticker = interval(Duration::from_millis(self.refresh_interval_ms));
        tokio::spawn(Self::collect_results(
            receiver,
            self.matches.clone(),
            self.current_task_id.clone(),
            self.top_item.clone(),
        ));
        loop {
            // ticker.tick().await;
            if event::poll(Duration::from_millis(self.refresh_interval_ms))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Esc => break,
                        KeyCode::Char(c) => {
                            self.query.push(c);
                            self.rescore_items(&sender).await;
                        }
                        KeyCode::Backspace => {
                            self.query.pop();
                            self.rescore_items(&sender).await;
                        }
                        KeyCode::Enter => {
                            execute!(self.stdout, Clear(ClearType::All),)?;
                            disable_raw_mode()?;
                            let lock = self.top_item.read().await;
                            if let Some(s) = &*lock {
                                println!("{}", s);
                            }
                            return Ok(());
                        }
                        _ => {}
                    }
                }
            }
            self.render().await.unwrap();
        }
        disable_raw_mode()?;
        Ok(())
    }

    async fn rescore_items(&mut self, sender: &Sender<Vec<ResultItem>>) {
        self.current_task_id.fetch_add(1, Ordering::SeqCst);
        let inputs = self.inputs.clone();
        let query = self.query.clone();
        let task_id = self.current_task_id.load(Ordering::SeqCst).clone();
        let current_task_id = self.current_task_id.clone();
        let sender = sender.clone();
        let mut lock = self.matches.write().await;
        lock.clear();
        tokio::task::spawn_blocking(move || {
            for chunk in inputs.chunks(100) {
                if task_id != current_task_id.load(Ordering::SeqCst) {
                    return;
                }
                let items = score_items(chunk, &query, task_id);
                if items.len() > 0 {
                    sender.blocking_send(items).unwrap();
                }
            }
        });
    }

    async fn collect_results(
        mut receiver: Receiver<Vec<ResultItem>>,
        matches: Arc<RwLock<Vec<ResultItem>>>,
        current_task_id: Arc<AtomicU64>,
        top_item: Arc<RwLock<Option<String>>>,
    ) {
        // channel will sends results in batches - assumes that empty vector will never be sent
        while let Some(batch) = receiver.recv().await {
            assert!(batch.len() > 0);
            // results in same batch will always come from the same scoring task
            let scoring_task_id = batch.first().unwrap().scoring_task_id;
            if scoring_task_id == current_task_id.load(Ordering::SeqCst) {
                let mut matches_lock = matches.write().await;
                matches_lock.extend_from_slice(&batch);
                matches_lock.sort_by_key(|result_item| result_item.score);
                let mut top_item_lock = top_item.write().await;
                *top_item_lock = Some(matches_lock.first().unwrap().content.clone());
            }
        }
    }

    async fn render(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // let items = self.matches.last().unwrap();
        // top_item = None;
        execute!(
            self.stdout,
            Clear(ClearType::All),
            MoveTo(0, 0),
            SetForegroundColor(Color::Green),
            Print(&self.prompt),
            ResetColor,
            Print(&self.query),
            SetForegroundColor(Color::White),
        )?;
        let matches = self.matches.read().await;
        for i in 0..self.match_display_limit {
            if i >= matches.len() {
                break;
            }
            execute!(
                self.stdout,
                MoveTo(0, (i + 1).try_into().unwrap()),
                Print(format!("{}: ", i)),
                Print(format!("{}\n", matches[i].content))
            )?;
        }
        execute!(
            self.stdout,
            MoveTo(self.query.len() as u16 + self.prompt.len() as u16, 0),
        )?;
        Ok(())
    }
}
