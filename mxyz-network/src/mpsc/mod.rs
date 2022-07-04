use crate::message::Message as MpscMessage; // TODO rename: Message -> MpscMessage
use std::sync::mpsc;

pub struct MpscSender {
    pub tx: mpsc::Sender<MpscMessage>,
}
impl MpscSender {
    pub fn new(tx: mpsc::Sender<MpscMessage>) -> Self {
        MpscSender { tx }
    }
}
unsafe impl Send for MpscSender {}
// unsafe impl Sync for MpscSender {}

pub struct MpscReceiver {
    pub rx: mpsc::Receiver<MpscMessage>,
}
impl MpscReceiver {
    pub fn new(rx: mpsc::Receiver<MpscMessage>) -> Self {
        MpscReceiver { rx }
    }
}
unsafe impl Send for MpscReceiver {}
// unsafe impl Sync for MpscReceiver {}

pub fn create_channel() -> (MpscSender, MpscReceiver) {
    let (tx, rx) = mpsc::channel::<MpscMessage>();

    let sender = MpscSender::new(tx);
    let receiver = MpscReceiver::new(rx);

    (sender, receiver)
}
