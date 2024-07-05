use aligned_sized::aligned_sized;
use anchor_lang::prelude::*;

use crate::QueueMetadata;

#[account(zero_copy)]
#[aligned_sized(anchor)]
#[derive(AnchorDeserialize, Debug)]
pub struct BatchedAddressQueueAccount {
    pub metadata: QueueMetadata,
    pub num_bloom_filters: u64,
}

pub struct BloomFilterWrapper {
    pub id: u8,
    pub offset_store: Offset,
    pub offset_value_store: Option<Offset>,
}

pub struct Offset {
    pub start: usize,
    pub end: usize,
}
