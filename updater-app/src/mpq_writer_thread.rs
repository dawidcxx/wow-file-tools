use crate::logic;
use once_cell::sync::Lazy;
use std::{
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    time::Duration,
};
use stormlib::MpqArchive;

#[derive(Debug)]
pub enum Message {
    Open(Sender<()>),
    WriteFile(String, Vec<u8>, Sender<()>),
    Close(Sender<()>),
    Exit(Sender<()>),
}

static MPQ_WRITER_SENDER: Lazy<Arc<Sender<Message>>> = Lazy::new(|| {
    let (sender, receiver) = mpsc::channel::<Message>();
    std::thread::spawn(move || {
        let mut archive: Option<stormlib::MpqArchive> = None;
        let mut times_opened = 0 as usize;
        let mut times_written = 0 as usize;

        for message in receiver {
            match message {
                Message::Open(reply) => {
                    println!("MPQThread: Opening MPQ archive");
                    if archive.is_some() {
                        println!("MPQThread: MPQ archive already open");
                        reply.send(()).expect("MPQThread: Failed to send reply");
                        continue;
                    }
                    let mpq_path = logic::get_mpq_path();
                    archive.replace(
                        MpqArchive::from_path(&mpq_path.to_string_lossy())
                            .expect("MPQThread: Failed to open MPQ archive"),
                    );
                    times_opened += 1;
                    reply.send(()).expect("MPQThread: Failed to send reply");
                }
                Message::WriteFile(file_path, file_content, reply) => {
                    println!("MPQThread: Writing file to MPQ archive: {}", file_path);
                    let archive = archive.as_mut().expect("MPQThread: MPQ archive not open");
                    archive
                        .write_file(&file_path, &file_content.as_slice())
                        .expect("MPQThread: Failed to write file to MPQ archive");
                    times_written += 1;
                    reply.send(()).expect("MPQThread: Failed to send reply");
                }
                Message::Close(reply) => {
                    println!("MPQThread: Closing MPQ archive. Open='{}'", archive.is_some());
                    archive.take();
                    reply.send(()).expect("MPQThread: Failed to send reply");
                }
                Message::Exit(reply) => {
                    reply.send(()).expect("MPQThread: Failed to send reply");
                    break;
                }
            }
        }
        println!(
            "MPQThread: Exiting. Stats: opened {} times, written {} times",
            times_opened, times_written
        );
    });

    Arc::new(sender)
});

pub fn open() {
    let (tx, rx) = mpsc::channel();
    send_to_mpq_writer_thread(Message::Open(tx));
    rx.recv_timeout(Duration::from_secs(1))
        .expect("Failed to get response from MPQ writer thread");
}

pub fn write_file(file_path: &String, file_content: Vec<u8>) {
    let (tx, rx) = mpsc::channel();
    send_to_mpq_writer_thread(Message::WriteFile(file_path.clone(), file_content, tx));
    rx.recv_timeout(Duration::from_secs(1))
        .expect("Failed to get response from MPQ writer thread");
}

pub fn close() {
    let (tx, rx) = mpsc::channel();
    send_to_mpq_writer_thread(Message::Close(tx));
    rx.recv_timeout(Duration::from_secs(1))
        .expect("Failed to get response from MPQ writer thread");
}

pub fn exit() {
    let (tx, rx) = mpsc::channel();
    send_to_mpq_writer_thread(Message::Exit(tx));
    rx.recv_timeout(Duration::from_secs(1))
        .expect("Failed to get response from MPQ writer thread");
}

fn send_to_mpq_writer_thread(msg: Message) {
    MPQ_WRITER_SENDER
        .send(msg)
        .expect("Failed to dispatch message to MPQ writer thread");
}
