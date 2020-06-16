use crate::env::{GENESIS_DIFF, BLOCK_TIME};


// Calculate the new difficulty given the height and previous block time.
// If the diff is being calculated for the genesis block return the default
// diff constant defined in the environment.
pub fn calculate_diff(height: u8, prev_diff: u8, prev_blocktime: u32) -> u8 {
    if height == 0 && prev_blocktime == 0 {
        return GENESIS_DIFF;
    }

    let upper_limit: u32 = (BLOCK_TIME as f32 * 1.25).floor() as u32;
    let lower_limit: u32 = (BLOCK_TIME as f32 * 0.75).floor() as u32;

    let diff = if prev_blocktime < lower_limit {
        prev_diff + 1
    }
    else if prev_blocktime > upper_limit {
        prev_diff - 1
    }
    else {
        prev_diff
    };

    return diff;
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn calculate_genesis_diff() {
        let height: u8 = 0;
        let prev_diff: u8 = 0;
        let prev_time: u32 = 0;

        assert_eq!(GENESIS_DIFF, calculate_diff(height, prev_diff, prev_time));
    }

    #[test]
    fn calculate_diff_static() {
        let height: u8 = 10;
        let prev_diff: u8 = 10;
        let prev_time: u32 = BLOCK_TIME;

        assert_eq!(prev_diff, calculate_diff(height, prev_diff, prev_time));
    }

    #[test]
    fn calculate_diff_increase() {
        let height: u8 = 10;
        let prev_diff: u8 = 10;
        let prev_time: u32 = (BLOCK_TIME as f32 * 0.7).floor() as u32;

        assert_eq!(prev_diff + 1, calculate_diff(height, prev_diff, prev_time));
    }

    #[test]
    fn calculate_diff_decrease() {
        let height: u8 = 10;
        let prev_diff: u8 = 10;
        let prev_time: u32 = (BLOCK_TIME as f32 * 1.3).floor() as u32;

        assert_eq!(prev_diff - 1, calculate_diff(height, prev_diff, prev_time));
    }
}