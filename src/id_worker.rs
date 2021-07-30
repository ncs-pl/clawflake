use chrono::Utc;
use parking_lot::Mutex;
use std::sync::Arc;

#[derive(Default)]
#[derive(Debug)]
pub struct IdWorker {
    epoch: i64,
    worker_id: i64,
    datacenter_id: i64,
    sequence: i64,
    time: Arc<Mutex<i64>>,
}

impl IdWorker {
    pub fn new(epoch: i64, worker_id: i64, datacenter_id: i64) -> IdWorker {
      IdWorker {
            epoch,
            worker_id,
            datacenter_id,
            sequence: 0,
            time: Arc::new(Mutex::new(0)),
        }
    }

    #[allow(dead_code)]
    pub fn epoch(&mut self, epoch: i64) -> &mut Self {
        self.epoch = epoch;
        self
    }

    #[allow(dead_code)]
    pub fn worker_id(&mut self, worker_id: i64) -> &mut Self {
        self.worker_id = worker_id;
        self
    }

    #[allow(dead_code)]
    pub fn datacenter_id(&mut self, datacenter_id: i64) -> &mut Self {
        self.datacenter_id = datacenter_id;
        self
    }

    pub fn next_id(&mut self) -> i64 {
        let mut last_timestamp = self.time.lock();
        let mut timestamp = self.get_time();
        if timestamp == *last_timestamp {
            self.sequence = (self.sequence + 1) & (-1 ^ (-1 << 12));
            if self.sequence == 0 && timestamp <= *last_timestamp {
                timestamp = self.get_time();
            }
        } else {
            self.sequence = 0;
        }
        *last_timestamp = timestamp;
        (timestamp << 22) | (self.worker_id << 17) | (self.datacenter_id << 12) | self.sequence
    }

    fn get_time(&self) -> i64 {
        Utc::now().timestamp_millis() - self.epoch
    }
}
