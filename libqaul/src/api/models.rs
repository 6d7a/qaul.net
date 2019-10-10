//! Service API exchange models

use std::fmt::{self, Debug, Formatter};

use identity::Identity;
use mime::Mime;

/// Convenience type for API functions
pub type QaulResult<T> = Result<T, QaulError>;

/// Service API error wrapper
#[derive(Debug, Clone, PartialEq)]
pub enum QaulError {
    /// Not authorised to perform this action
    NotAuthorised,
    /// The desired user was not known
    UnknownUser,
    /// Invalid search query
    InvalidQuery,
    /// Invalid payload (probably too big)
    InvalidPayload,
    /// A function callback timed out
    CallbackTimeout,
}

/// A security token to authenticate sessions
#[derive(Clone, PartialEq, Eq)]
pub struct Token(String);

impl Debug for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "<TOKEN>")
    }
}

impl From<String> for Token {
    fn from(s: String) -> Self {
        assert!(s.len() == 64);
        Token(s)
    }
}

/// A wrapper around user authentication state
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UserAuth {
    /// A user ID which has not been verified
    Untrusted(Identity),
    /// The user ID of the currently logged-in user
    Trusted(Identity, String),
}

impl UserAuth {
    /// Returns an error if the UserAuth isn't Trusted.
    pub fn trusted(self) -> QaulResult<(Identity, String)> {
        match self {
            UserAuth::Trusted(id, s) => Ok((id, s)),
            UserAuth::Untrusted(_) => Err(QaulError::NotAuthorised),
        }
    }

    /// Returns the interior identity, regardless of trust status.
    pub fn identity(self) -> Identity {
        match self {
            UserAuth::Trusted(id, _) => id,
            UserAuth::Untrusted(id) => id,
        }
    }

    /// Returns the interior identity as an `Untrusted`, regardless of trust status.
    pub fn as_untrusted(&self) -> Self {
        UserAuth::Untrusted(self.clone().identity())
    }
}

/// Signature trust information embedded into service messages
pub enum SigTrust {
    /// A verified signature by a known contact
    Trusted(Identity),
    /// An unverified signature by a known contact
    /// (pubkey not available!)
    Unverified(Identity),
    /// A fraudulent signature
    Invalid,
}

/// A service message
///
/// Differs from the `RATMAN` abstraction for messages
/// because it's signature has already been verified.
/// Instead of delivering the raw signature to a service,
/// this message only embeds validity information.
///
/// This makes it easier for service authors to trust
/// data provided by `libqaul`, without having to do
/// calls into some crypto library themselves.
///
/// In comparison to the `RATMAN` message, the `associator`
/// has also been removed because at this stage, only the
/// relevant related service is being handed a message anyway.
pub struct Message {
    pub sender: Identity,
    pub recipient: Recipient,
    pub payload: Vec<u8>,
    pub signature: SigTrust,
}

/// Service message recipient
///
/// A recipient is either a single user or the entire network.  The
/// "flood" mechanic is passed through to `RATMAN`, which might
/// implement this in the networking module, or emulate
/// it. Performance may vary.
pub enum Recipient {
    /// A single user, known to this node
    User(Identity),
    /// A collection of users, sometimes called a Group
    Group(Vec<Identity>),
    /// Addressed to nobody, flooded into the network
    Flood,
}

/// Local file abstraction
pub struct File {
    pub name: String,
    pub mime: Mime,
    pub data: Option<Vec<u8>>,
}

/// Describe a file's lifecycle
///
/// Not to be confused with `FileFilter`, which is part of public API
/// functions to allow users to easily filter for only certain types
/// of file data.
///
/// Filter functions then take a `Filter` and return a `Meta`.
pub enum FileMeta {
    /// Files owned by the current user
    Local(File),
    /// Network files, fully locally mirrored
    Available(File),
    /// Network files, still downloading
    InProgress {
        size: usize,
        local: usize,
        stalled: bool,
    },
    /// A network advertised file that hasn't started downloading
    Advertised { size: usize },
}

/// Describe a file's lifecycle
///
/// Filter functions for each time exist and enable
/// different sub-services based on which phase they
/// aim for.
pub enum FileFilter {
    Local,
    Available,
    InProgress,
}
