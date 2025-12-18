use std::{
    io::BufReader,
    process::ChildStderr,
    sync::mpsc::{sync_channel, Receiver, SyncSender},
    thread::JoinHandle,
};

use super::{child::CrocChild, parser::CrocParser, CrocEvent};

pub struct CrocIterator {
    rx: Receiver<CrocEvent>,
    tx: Option<SyncSender<CrocEvent>>,
}

impl CrocIterator {
    pub fn new(child: &mut CrocChild) -> Result<Self, &'static str> {
        let stderr = child
            .take_stderr()
            .ok_or("Failed to take stderr from CrocChild")?;

        let (tx, rx) = sync_channel::<CrocEvent>(0);
        spawn_stderr_thread(stderr, tx.clone());
        Ok(Self { rx, tx: Some(tx) })
    }
}

impl Iterator for CrocIterator {
    type Item = CrocEvent;

    fn next(&mut self) -> Option<Self::Item> {
        let item = self.rx.recv().ok();

        if let Some(CrocEvent::Done) = &item {
            // Close the sender to signal no more events will be sent
            self.tx.take();
        }

        item
    }
}

pub fn spawn_stderr_thread(stderr: ChildStderr, tx: SyncSender<CrocEvent>) -> JoinHandle<()> {
    std::thread::spawn(move || {
        let reader = BufReader::new(stderr);
        let mut parser = CrocParser::new(reader);

        loop {
            match parser.parse_next_event() {
                Some(CrocEvent::Done) => {
                    tx.send(CrocEvent::Done).ok();
                    break;
                }
                Some(event) => tx.send(event).ok(),
                None => break,
            };
        }
    })
}
