use anchor_lang::prelude::*;

/// Aggregated statistics for priority fees
#[account]
pub struct PriorityFeesStats {
    pub authority: Pubkey,
    pub last_updated: i64,      // Timestamp of the Last update
    pub sample_count: u32,      // Amount of data used for calculations 
    pub sum: u128,              // Sum of all observed fees
    pub min: u64,               // Minimum fee
    pub max: u64,               // Maximum fee
    pub last_slot: u64,         // Last value
    pub bump: u8,
}

impl PriorityFeesStats {
    pub const LEN: usize = 8 +  // Anchor discriminator
        32 +                    // authority
        8 +                     // last_updated
        4 +                     // sample_count
        16 +                    // sum
        8 +                     // min
        8 +                     // max
        8 +                     // last_slot
        1;                      // bump

    /// Add a new priority fee sample and update stats
    pub fn add_sample(&mut self, slot: u64, fee: u64, timestamp: i64) {
        self.last_updated = timestamp;
        self.last_slot = slot;
        self.sample_count = self.sample_count.saturating_add(1);
        self.sum = self.sum.saturating_add(fee as u128);
        if self.sample_count == 1 {
            self.min = fee;
            self.max = fee;
        } else {
            if fee < self.min { self.min = fee; }
            if fee > self.max { self.max = fee; }
        }
    }

    /// Get average fee (if any samples)
    pub fn average(&self) -> Option<u64> {
        if self.sample_count == 0 {
            None
        } else {
            Some((self.sum / self.sample_count as u128) as u64)
        }
    }
}
