use std::path::PathBuf;

#[derive(Clone, Debug)]
pub enum TrackType {
    RegQueueTrack(PathBuf),
    ExQueueTrack(PathBuf),
    None
}

// TODO: Change to queue to hold track id and play lookup in db
#[derive(Debug)]
pub struct TrackQueue {
    pub reg_queue: Vec<PathBuf>,
    pub ex_queue: Vec<PathBuf>, 
    pub curr_track: TrackType,
    pub played_tracks: Vec<PathBuf>
}

impl TrackQueue {
    pub fn new() -> Self {
        TrackQueue {
            reg_queue: Vec::new(),
            ex_queue: Vec::new(),
            curr_track: TrackType::None,
            played_tracks: Vec::new(),
        }
    }

    pub fn play_next(&mut self, p: PathBuf) {
        self.ex_queue.insert(0, p);
    }

    pub fn add_to_queue(&mut self, p: PathBuf) {
        self.ex_queue.push(p);
    }

    pub fn add_to_reg_queue(&mut self, p: PathBuf) {
        self.reg_queue.push(p);
    }

    pub fn get_curr_track(&self) -> Option<PathBuf> {
        match self.curr_track.clone() {
            TrackType::RegQueueTrack(t) => Some(t),
            TrackType::ExQueueTrack(t) => Some(t),
            TrackType::None => None
        }
    }

    pub fn next_track(&mut self) {
        // move curr track to played tracks vec
        if let TrackType::RegQueueTrack(t) = self.curr_track.clone() {
            self.played_tracks.push(t);
        }
        self.curr_track = TrackType::None;

        // get next track from explicit queue first, if empty look at regular queue
        // if both empty, reset regular queue to played tracks and call next track again
        if self.ex_queue.len() > 0 {
            self.curr_track = TrackType::ExQueueTrack(self.ex_queue.remove(0));
        } else if self.reg_queue.len() > 0 {
            self.curr_track = TrackType::RegQueueTrack(self.reg_queue.remove(0));
        } else {
            self.reg_queue = self.played_tracks.clone();
            self.played_tracks = Vec::new();
            self.next_track();
        }
        dbg!(self);
    }

    pub fn prev_track(&mut self) {
        if self.played_tracks.len() > 0 {
            match self.curr_track.clone() {
                TrackType::RegQueueTrack(t) => { self.reg_queue.insert(0, t) },
                TrackType::ExQueueTrack(t) => { self.ex_queue.insert(0, t) },
                TrackType::None => {}
            }
            self.curr_track = TrackType::RegQueueTrack(self.played_tracks.pop().unwrap());
        }
        dbg!(self);
    }
}
