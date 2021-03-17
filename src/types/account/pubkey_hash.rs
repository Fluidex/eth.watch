/// Hash of the account's owner public key.
///
/// This is an essential type used within Fluidex network to authorize transaction author
/// to perform an operation.
///
/// `PubKeyHash` is calculated as the Rescue hash of the public key byte sequence.
#[derive(Copy, Clone, PartialEq, Default, Eq, Hash, PartialOrd, Ord)]
pub struct PubKeyHash {
    // pub data: [u8; params::FR_ADDRESS_LEN],
}
