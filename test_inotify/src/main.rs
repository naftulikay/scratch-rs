use futures::StreamExt;
use inotify::{EventMask, Inotify, WatchMask};
use std::path::PathBuf;
use tracing::{debug, info, warn};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

const EVENT_MASK_KINDS: &'static [EventMask] = &[
    EventMask::ACCESS,
    EventMask::ATTRIB,
    EventMask::CLOSE_NOWRITE,
    EventMask::CLOSE_WRITE,
    EventMask::CREATE,
    EventMask::DELETE,
    EventMask::DELETE_SELF,
    EventMask::MODIFY,
    EventMask::MOVE_SELF,
    EventMask::MOVED_FROM,
    EventMask::MOVED_TO,
    EventMask::OPEN,
    EventMask::IGNORED,
    EventMask::ISDIR,
    EventMask::Q_OVERFLOW,
    EventMask::UNMOUNT,
];

#[derive(Clone, Debug)]
enum EventKind {
    Access,
    Attrib,
    CloseWrite,
    CloseNoWrite,
    Create,
    Delete,
    DeleteSelf,
    Modify,
    MoveSelf,
    MovedFrom,
    MovedTo,
    Open,
    Ignored,
    IsDir,
    QueueOverflow,
    Unmount,
    Unknown(EventMask),
}

impl EventKind {
    pub fn to_vec(mask: &EventMask) -> Vec<EventKind> {
        EVENT_MASK_KINDS
            .iter()
            .filter(|&m| mask.intersects(*m))
            .map(|m| EventKind::from_mask_bit(m))
            .collect()
    }

    fn from_mask_bit(mask: &EventMask) -> EventKind {
        if mask.intersects(EventMask::ACCESS) {
            EventKind::Access
        } else if mask.intersects(EventMask::ATTRIB) {
            EventKind::Attrib
        } else if mask.intersects(EventMask::CLOSE_NOWRITE) {
            EventKind::CloseNoWrite
        } else if mask.intersects(EventMask::CLOSE_WRITE) {
            EventKind::CloseWrite
        } else if mask.intersects(EventMask::CREATE) {
            EventKind::Create
        } else if mask.intersects(EventMask::DELETE) {
            EventKind::Delete
        } else if mask.intersects(EventMask::DELETE_SELF) {
            EventKind::DeleteSelf
        } else if mask.intersects(EventMask::MODIFY) {
            EventKind::Modify
        } else if mask.intersects(EventMask::MOVE_SELF) {
            EventKind::MoveSelf
        } else if mask.intersects(EventMask::MOVED_FROM) {
            EventKind::MovedFrom
        } else if mask.intersects(EventMask::MOVED_TO) {
            EventKind::MovedTo
        } else if mask.intersects(EventMask::OPEN) {
            EventKind::Open
        } else if mask.intersects(EventMask::IGNORED) {
            EventKind::Ignored
        } else if mask.intersects(EventMask::ISDIR) {
            EventKind::IsDir
        } else if mask.intersects(EventMask::Q_OVERFLOW) {
            EventKind::QueueOverflow
        } else if mask.intersects(EventMask::UNMOUNT) {
            EventKind::Unmount
        } else {
            EventKind::Unknown(mask.clone())
        }
    }
}

#[cfg(target_os = "linux")]
#[tokio::main]
async fn main() {
    tracing::subscriber::set_global_default(
        Registry::default().with(tracing_subscriber::fmt::layer().pretty()),
    )
    .expect("unable to configure logging");

    let notifier = Inotify::init().expect("unable to initialize inotify");

    debug!("inotify initialized");

    let device_dir = PathBuf::from("/dev/snd");

    let watch = notifier
        .watches()
        .add(&device_dir, WatchMask::CREATE | WatchMask::DELETE)
        .expect("unable to add a watch to /dev/snd");

    debug!(wd = watch.get_watch_descriptor_id(), "watch created");

    info!("Listening for events...");

    let mut event_stream = notifier
        // 4KiB buffer space
        .into_event_stream(vec![0; 4096])
        .expect("unable to convert notifier into event stream");

    while let Some(event) = event_stream.next().await {
        let event = match event {
            Ok(e) => e,
            Err(e) => {
                warn!(
                    "Received error when attempting to listen for events: {:?}",
                    e
                );
                continue;
            }
        };

        let path = if let Some(name) = event.name {
            device_dir
                .join(PathBuf::from(name))
                .to_string_lossy()
                .to_string()
        } else {
            "(none)".to_string()
        };

        let flags = EventKind::to_vec(&event.mask)
            .into_iter()
            .map(|k| format!("{:?}", k))
            .collect::<Vec<String>>()
            .join(",");

        let cookie = if event.cookie > 0 {
            format!("{:X}", event.cookie)
        } else {
            "(none)".to_string()
        };

        info!(
            wd = event.wd.get_watch_descriptor_id(),
            path, flags, cookie, "Event received"
        );
    }
}
