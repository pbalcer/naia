use naia_socket_shared::PacketReader;

use crate::{diff_mask::DiffMask, impls::ProtocolType};

/// A Replica is a Message/Component, or otherwise, a container
/// of Properties that can be scoped, tracked, and synced, with a remote host
pub trait Replicate<P: ProtocolType>: Sync + Send + 'static {

    /// Reads data from an incoming packet, sufficient to sync the in-memory
    /// Component with it's replica on the Server
    fn read_partial(&mut self, diff_mask: &DiffMask, reader: &mut PacketReader, packet_index: u16);

    /// Returns self as a Protocol
    fn to_protocol(self) -> P;
    /// Gets the TypeId of the Message/Component, used to map to a
    /// registered ProtocolType
    fn get_kind(&self) -> P::Kind;
    /// Writes data into an outgoing byte stream, sufficient to completely
    /// recreate the Message on the other host
    fn write(&self, out_bytes: &mut Vec<u8>);
}

//TODO: do we really need another trait here?
/// Handles equality of Messages/Components.. can't just derive
/// PartialEq because we want to only compare Properties
pub trait ReplicateEq<P: ProtocolType>: Replicate<P> {
    /// Compare with properties in another Replica
    fn equals(&self, other: &Self) -> bool;
    /// Sets the current Replica to the state of another Replica of the
    /// same type
    fn mirror(&mut self, other: &Self);
    /// Gets a copy of the Message/Component
    fn copy(&self) -> Self;
}