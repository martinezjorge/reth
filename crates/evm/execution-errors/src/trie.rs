//! Errors when computing the state root.

use alloy_primitives::B256;
use nybbles::Nibbles;
use reth_storage_errors::{db::DatabaseError, provider::ProviderError};
use derive_more::{Display, From};

#[cfg(not(feature = "std"))]
use alloc::string::ToString;

/// State root errors.
#[derive(Display, Debug, From, PartialEq, Eq, Clone)]
pub enum StateRootError {
    /// Internal database error.
    Database(DatabaseError),
    /// Storage root error.
    StorageRootError(StorageRootError),
}

#[cfg(feature = "std")]
impl std::error::Error for StateRootError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(source) => {
                std::error::Error::source(source)
            },
            Self::StorageRootError(source) => {
                std::error::Error::source(source)
            }
        }
    }
}

impl From<StateRootError> for DatabaseError {
    fn from(err: StateRootError) -> Self {
        match err {
            StateRootError::Database(err) |
            StateRootError::StorageRootError(StorageRootError::Database(err)) => err,
        }
    }
}

/// Storage root error.
#[derive(Display, From, PartialEq, Eq, Clone, Debug)]
pub enum StorageRootError {
    /// Internal database error.
    Database(DatabaseError),
}

#[cfg(feature = "std")]
impl std::error::Error for StorageRootError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(source) => {
                std::error::Error::source(source)
            }
        }
    }
}

impl From<StorageRootError> for DatabaseError {
    fn from(err: StorageRootError) -> Self {
        match err {
            StorageRootError::Database(err) => err,
        }
    }
}

/// State proof errors.
#[derive(Display, Debug, From, PartialEq, Eq, Clone)]
pub enum StateProofError {
    /// Internal database error.
    Database(DatabaseError),
    /// RLP decoding error.
    Rlp(alloy_rlp::Error),
}

#[cfg(feature = "std")]
impl std::error::Error for StateProofError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Database(source) => {
                std::error::Error::source(source)
            },
            Self::Rlp(source) => {
                std::error::Error::source(source)
            }
        }
    }
}

impl From<StateProofError> for ProviderError {
    fn from(value: StateProofError) -> Self {
        match value {
            StateProofError::Database(error) => Self::Database(error),
            StateProofError::Rlp(error) => Self::Rlp(error),
        }
    }
}

/// Trie witness errors.
#[derive(Display, Debug, From, PartialEq, Eq, Clone)]
pub enum TrieWitnessError {
    /// Error gather proofs.
    Proof(StateProofError),
    /// RLP decoding error.
    Rlp(alloy_rlp::Error),
    /// Missing storage multiproof.
    #[display(fmt = "missing storage multiproof for {_0}")]
    MissingStorageMultiProof(B256),
    /// Missing account.
    #[from(ignore)]
    #[display(fmt = "missing account {_0}")]
    MissingAccount(B256),
    /// Missing target node.
    #[display(fmt = "target node missing from proof {_0:?}")]
    MissingTargetNode(Nibbles),
}

#[cfg(feature = "std")]
impl std::error::Error for TrieWitnessError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Proof(source) => {
                std::error::Error::source(source)
            },
            Self::Rlp(source) => {
                std::error::Error::source(source)
            },
            _ => Option::None
        }
    }
}

impl From<TrieWitnessError> for ProviderError {
    fn from(value: TrieWitnessError) -> Self {
        Self::TrieWitnessError(value.to_string())
    }
}
