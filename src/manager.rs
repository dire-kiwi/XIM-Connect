use crate::xim::{XIMManager, XIMEvent};
use crate::config::Config;
use crate::window;
use winapi::shared::windef::HWND;
use std::thread;
use std::sync::mpsc::{Sender, Receiver, channel};
use std::sync::Arc;
use send_wrapper::SendWrapper;
pub enum ManagerEvent {
    WindowCreated(i32)
}
pub struct Manager {
    overlay_hwnd: Option<SendWrapper<HWND>>,    
}


impl Manager {
    pub fn start() {
        let manager = Manager {
            overlay_hwnd: None,
        };

        
        let (tx, rx) = channel::<ManagerEvent>();
        let (xim_tx, xim_rx) = channel::<Vec<XIMEvent>>();
        
        let handler = manager.process_raw_input(&xim_tx, &tx);
        manager.process_xim(xim_rx);
        handler.join();
    }

    pub fn process_raw_input(&self, xim_tx: &Sender<Vec<XIMEvent>>, manager_tx: &Sender<ManagerEvent>) -> thread::JoinHandle<()> {
        let tx = xim_tx.clone();
        let tx1 = manager_tx.clone();
        thread::spawn(move || window::process_events(tx, tx1))
    }

    pub fn process_xim(&self, xim_rx: Receiver<Vec<XIMEvent>>) -> thread::JoinHandle<()> {
        std::thread::spawn(move || {
            let mut xim = XIMManager::new();
            unsafe{
                xim.connect();
                loop {
                    xim.send_input(xim_rx.recv().expect("error found")); 
                }   
            } 
        })
        
    }
}