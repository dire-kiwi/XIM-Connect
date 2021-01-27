use crate::xim::{XIMManager, XIMEvent};
use crate::config::Config;
use crate::window;
use winapi::shared::windef::HWND;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;

pub enum ManagerEvent {
    WindowCreated(i32)
}
pub struct Manager {
    xim: Arc<XIMManager>,
    config: Config,
    overlay_hwnd: Option<i32>,    
}


impl Manager {
    pub fn start() {
        let manager = Manager {
            xim: Arc::new(XIMManager::new()),
            config: Config::read_config(),
            overlay_hwnd: None,
        };
        let (tx, rx) = channel::<ManagerEvent>();
        let (xim_tx, xim_rx) = channel::<XIMEvent>();
        
        let handler = manager.process_raw_input(&xim_tx, &tx);

        handler.join();
    }

    pub fn process_raw_input(&self, xim_tx: &Sender<XIMEvent>, manager_tx: &Sender<ManagerEvent>) -> thread::JoinHandle<()> {
        let tx = xim_tx.clone();
        let tx1 = manager_tx.clone();
        thread::spawn(move || window::process_events(tx, tx1))
    }
}