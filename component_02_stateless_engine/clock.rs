
pub fn global_clock_from_block(block_number: u64) -> u128 {
    const GENESIS_TIMESTAMP: u128 = 1700000000000; // Replace with actual genesis timestamp in ms
    GENESIS_TIMESTAMP + (block_number as u128)
}
