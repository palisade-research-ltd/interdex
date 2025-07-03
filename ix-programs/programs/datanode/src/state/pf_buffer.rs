use anchor_lang::prelude::*;

/// Circular buffer for recent priority fees
#[account]
pub struct PriorityFeesBuffer {
    pub authority: Pubkey,
    pub last_updated: i64,
    pub current_index: u16,
    pub slots: [u64; 16],       // Last 16 slots
    pub fees: [u64; 16],        // Last 16 priority fees (micro-lamports)
    pub is_full: bool,          // True when buffer is full
    pub bump: u8,               // Extra memory bump to avoid collisions
}

impl PriorityFeesBuffer {
    pub const LEN: usize = 8 +  // Anchor discriminator
        32 +                    // authority
        8 +                     // last_updated
        2 +                     // current_index
        8 * 16 +                // slots
        8 * 16 +                // fees
        1 +                     // is_full
        1;                      // bump

    /// Add new priority fee record to circular buffer
    pub fn add_priority_fee(&mut self, slot: u64, fee: u64, timestamp: i64) {
        self.slots[self.current_index as usize] = slot;
        self.fees[self.current_index as usize] = fee;
        self.last_updated = timestamp;

        self.current_index = (self.current_index + 1) % 16;
        if self.current_index == 0 && !self.is_full {
            self.is_full = true;
        }
    }

    /// Get recent fees (most recent N)
    pub fn get_recent_fees(&self, count: usize) -> (&[u64], &[u64]) {
        let available_count = if self.is_full { 16 } else { self.current_index as usize };
        let take_count = count.min(available_count);
        (&self.slots[..take_count], &self.fees[..take_count])
    }
}
