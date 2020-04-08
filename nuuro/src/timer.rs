use std::collections::HashMap;
use std::sync::Mutex;

type TimerId = i64;

lazy_static! {
    static ref TIMERS_INSTANCES: Mutex<HashMap<TimerId, TimerInstance>> =
        Mutex::new(HashMap::new());
}

static mut TIMERS_ID: TimerId = 0;

#[derive(Clone)]
/// Structure that allows calculating the time elapsed since its initialization
/// or determining whether a certain period of time has elapsed.
pub struct Timer {
    id: TimerId,
}

#[allow(dead_code)]
impl Timer {
    /// Initialize a new `Timer`. If the `expiration_time` parameter equals 0,
    /// an endless` Timer` is started; if the parameter has any value
    /// (in milliseconds), a new `Timer` will start which will expire upon
    /// reaching this period of time. The `timer_active` parameter will
    /// determine if the` Timer` will start at once when initialized
    /// (*note:* if it is an endless timer, it will always be initialized started).
    pub fn new(expiration_time: u64, start_active: bool) -> Timer {
        unsafe {
            TIMERS_ID += 1;
        }

        let timer = TimerInstance::new(expiration_time, start_active, unsafe { TIMERS_ID });

        unsafe {
            TIMERS_INSTANCES.lock().unwrap().insert(TIMERS_ID, timer);
        }

        Timer {
            id: unsafe { TIMERS_ID },
        }
    }

    /// Reset the elapsed time of the timer to 0, and if it has not started before,
    /// the time will begin to elapse.
    pub fn reset(&self) {
        TIMERS_INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.id)
            .unwrap()
            .reset();
    }

    /// Set a new expiration time for the timer and activation on start.
    pub fn set_time(&self, expiration_time: u64, start_active: bool) {
        TIMERS_INSTANCES
            .lock()
            .unwrap()
            .get_mut(&self.id)
            .unwrap()
            .set_time(expiration_time, start_active);
    }

    /// Get the timer status. If the expiration time has not been reached, the
    /// timer will be active.
    pub fn active(&self) -> bool {
        TIMERS_INSTANCES
            .lock()
            .unwrap()
            .get(&self.id)
            .unwrap()
            .active()
    }

    /// Indicates if the timer has expired.
    pub fn expired(&self) -> bool {
        TIMERS_INSTANCES
            .lock()
            .unwrap()
            .get(&self.id)
            .unwrap()
            .expired()
    }

    /// Returns the time elapsed since the initialization of the last timer cycle.
    pub fn elapsed(&self) -> u64 {
        TIMERS_INSTANCES
            .lock()
            .unwrap()
            .get(&self.id)
            .unwrap()
            .elapsed()
    }
}

impl Drop for Timer {
    fn drop(&mut self) {
        TIMERS_INSTANCES
            .lock()
            .unwrap()
            .retain(|&t, _| t != self.id);
    }
}

struct TimerInstance {
    current_time: u64,
    expiration_time: u64,
    endless: bool,

    pub id: TimerId,
}

#[allow(dead_code)]
impl TimerInstance {
    pub fn new(expiration_time: u64, start_active: bool, id: TimerId) -> TimerInstance {
        TimerInstance {
            current_time: if start_active { 0 } else { expiration_time },
            expiration_time,
            endless: expiration_time == 0,

            id,
        }
    }

    pub fn reset(&mut self) {
        self.current_time = 0;
    }
    pub fn set_time(&mut self, expiration_time: u64, start_active: bool) {
        self.current_time = if start_active { 0 } else { expiration_time };
        self.expiration_time = expiration_time;
        self.endless = expiration_time == 0;
    }

    pub fn active(&self) -> bool {
        if !self.endless {
            return self.current_time < self.expiration_time;
        }
        true
    }
    pub fn expired(&self) -> bool {
        !self.active()
    }

    pub fn elapsed(&self) -> u64 {
        self.current_time
    }

    pub fn is_endless(&self) -> bool {
        self.endless
    }

    pub fn set_endless(&mut self, endless: bool) {
        self.endless = endless;
    }

    pub fn update(&mut self, elapsed_time: f64) {
        let elapsed_time = (elapsed_time * 1000.0) as u64;
        if self.active() {
            self.current_time += elapsed_time;
        } else {
            self.current_time = self.expiration_time;
        }
    }
}

pub(crate) fn update_all(elapsed_time: f64) {
    for timer in TIMERS_INSTANCES.lock().unwrap().values_mut() {
        timer.update(elapsed_time);
    }
}
