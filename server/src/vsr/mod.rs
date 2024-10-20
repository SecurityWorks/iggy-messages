pub(crate) mod client_table;
pub(crate) mod replica;
pub(crate) mod status;
pub(crate) mod header;
pub(crate) mod message;

// Types
pub(crate) type OpNumber = u64;
pub(crate) type CommitNumber = u64;
pub(crate) type ViewNumber = u32;
pub(crate) type Version = usize;

// TODO: const generics ?
pub(crate) struct QuorumCounter<T> {
    value: T,
    acks: usize,
    quorum: bool,
}

impl<T> Default for QuorumCounter<T>
where
    T: Default,
{
    fn default() -> Self {
        Self {
            value: Default::default(),
            acks: Default::default(),
            quorum: Default::default(),
        }
    }
}
