use std::{
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

use color_eyre::Result;
use crossterm::event::{self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
use ratatui::layout::Size;

/// Terminal events.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
    /// The tick event sent at a regular interval.
    ///
    /// It's useful for updating the application state at a regular interval, or for triggering
    /// animations.
    Tick,

    /// Key press.
    ///
    /// This event is sent when a key is pressed. Crossterm can distinguish between key press and
    /// key release events, but this event only sends key press events.
    KeyPress(KeyEvent),

    /// Mouse click/scroll.
    Mouse(MouseEvent),

    /// Terminal resize.
    Resize(Size),

    /// An event that isn't relevant and will be ignored and not sent to the application.
    Ignored,
}

/// An event handler that listens for events from the terminal and converts them into a stream of
/// [`Event`]s.
#[derive(Debug)]
pub struct EventHandler {
    /// Event receiver channel.
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    /// Create a new event handler.
    ///
    /// This function spawns two threads:
    /// - A tick thread that sends a tick event at a regular interval defined by
    ///   `DEFAULT_TICK_RATE`.
    /// - An event handler thread that listens for events from the terminal and sends them to the
    ///  event handler.
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        let _tick_thread = spawn_tick_thread(sender.clone());
        let _crossterm_events_thread = spawn_event_handler_thread(sender.clone());
        Self { receiver }
    }

    /// Receive the next event from the handler thread.
    ///
    /// This function will always block the current thread if there'ss no data available and it's
    /// possible for more data to be sent.
    pub fn next(&self) -> Result<Event> {
        Ok(self.receiver.recv()?)
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// The default tick rate
pub const DEFAULT_TICK_RATE: Duration = Duration::from_millis(100);

/// Spawn the tick thread.
///
/// This thread sends a tick event at a regular interval. The interval is defined by
/// `DEFAULT_TICK_RATE`.
fn spawn_tick_thread(tx: mpsc::Sender<Event>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut previous_tick = Instant::now();
        loop {
            // try to make the period between ticks as close to `DEFAULT_TICK_RATE` as possible
            // even if the tick event takes a long time to process
            let next_tick = previous_tick + DEFAULT_TICK_RATE;
            thread::sleep(next_tick.saturating_duration_since(Instant::now()));
            previous_tick = next_tick;

            if tx.send(Event::Tick).is_err() {
                // send errors indicate that the receiver is not accepting events, which means the
                // app is shutting down
                break;
            }
        }
    })
}

/// Spawn the event handler thread.
///
/// This thread listens for events from the terminal and sends them to the event handler.
///
/// This might need to have a way to stop this if the app needs to spawn a new process or something
/// like that.
fn spawn_event_handler_thread(tx: mpsc::Sender<Event>) -> thread::JoinHandle<()> {
    thread::spawn(move || loop {
        if event::poll(Duration::from_millis(100)).expect("no events available") {
            let event = event::read().expect("unable to read event");
            match event.into() {
                Event::Ignored => {
                    // ignore events that can't be converted
                }
                event => {
                    if tx.send(event).is_err() {
                        // send errors indicate that the receiver is not accepting events, which
                        // means the app is shutting down
                        break;
                    }
                }
            }
        }
    })
}

impl From<CrosstermEvent> for Event {
    fn from(event: CrosstermEvent) -> Self {
        match event {
            // Only handle key press events. Ignore key release / repeat events
            CrosstermEvent::Key(e) if e.kind == KeyEventKind::Press => Event::KeyPress(e),
            CrosstermEvent::Key(_) => Event::Ignored,
            CrosstermEvent::Mouse(e) => Event::Mouse(e),
            CrosstermEvent::Resize(w, h) => Event::Resize(Size::new(w, h)),
            // ignore these events for now
            CrosstermEvent::FocusGained => Event::Ignored,
            CrosstermEvent::FocusLost => Event::Ignored,
            CrosstermEvent::Paste(_) => Event::Ignored,
        }
    }
}
