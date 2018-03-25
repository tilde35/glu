use std::time::Instant;

pub struct TimeStep {
    freq_nanos: u64,
    max_missed: u32,
    last_inst: Instant,
    elapsed_nanos: u64,
}

impl TimeStep {
    pub fn for_freq_ms(freq_msec: u32) -> Self {
        Self {
            freq_nanos: (freq_msec as u64) * 1_000_000, // Convert to nanoseconds
            max_missed: 1,
            last_inst: Instant::now(),
            elapsed_nanos: 0,
        }
    }
    pub fn max_missed_steps_before_discard(mut self, max_miss: u32) -> Self {
        self.max_missed = max_miss;
        self
    }

    fn update_elapsed(&mut self) {
        let now = Instant::now();
        let e = now.duration_since(self.last_inst);

        // 586,298,884 elapse, before 1500,000,000
        self.last_inst = now;
        self.elapsed_nanos += (e.as_secs() * 1_000_000_000) + (e.subsec_nanos() as u64);
    }

    pub fn tick<F>(&mut self, mut callback: F)
    where
        F: FnMut(),
    {
        self.update_elapsed();

        let mut count = 0;
        while self.elapsed_nanos >= self.freq_nanos {
            self.elapsed_nanos -= self.freq_nanos;

            callback();

            if count >= self.max_missed && self.elapsed_nanos >= self.freq_nanos {
                // Reached maximum missed steps and there is still more to go
                // Reset to zero and discontinue processing
                self.elapsed_nanos = 0;
                return;
            }
            count += 1;
        }
    }
}
