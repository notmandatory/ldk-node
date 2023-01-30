use std::fmt;

#[derive(Debug)]
/// An error that possibly needs to be handled by the user.
pub enum Error {
	/// Returned when trying to start [`crate::Node`] while it is already running.
	AlreadyRunning,
	/// Returned when trying to stop [`crate::Node`] while it is not running.
	NotRunning,
	/// The funding transaction could not be created.
	FundingTxCreationFailed,
	/// A network connection has been closed.
	ConnectionFailed,
	/// The given address is invalid.
	AddressInvalid,
	/// The given public key is invalid.
	PublicKeyInvalid,
	/// The given payment hash is invalid.
	PaymentHashInvalid,
	/// Payment of the given invoice has already been intiated.
	NonUniquePaymentHash,
	/// The given invoice is invalid.
	InvoiceInvalid,
	/// Invoice creation failed.
	InvoiceCreationFailed,
	/// The given channel ID is invalid.
	ChannelIdInvalid,
	/// The given network is invalid.
	NetworkInvalid,
	/// No route for the given target could be found.
	RoutingFailed,
	/// A given peer info could not be parsed.
	PeerInfoParseFailed,
	/// A channel could not be opened.
	ChannelCreationFailed,
	/// A channel could not be closed.
	ChannelClosingFailed,
	/// Persistence failed.
	PersistenceFailed,
	/// A wallet operation failed.
	WalletOperationFailed,
	/// A siging operation failed.
	WalletSigningFailed,
	/// A transaction sync operation failed.
	TxSyncFailed,
}

impl fmt::Display for Error {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			Self::AlreadyRunning => write!(f, "Node is already running."),
			Self::NotRunning => write!(f, "Node is not running."),
			Self::FundingTxCreationFailed => write!(f, "Funding transaction could not be created."),
			Self::ConnectionFailed => write!(f, "Network connection closed."),
			Self::AddressInvalid => write!(f, "The given address is invalid."),
			Self::PublicKeyInvalid => write!(f, "The given public key is invalid."),
			Self::PaymentHashInvalid => write!(f, "The given payment hash is invalid."),
			Self::NonUniquePaymentHash => write!(f, "An invoice must not get payed twice."),
			Self::InvoiceInvalid => write!(f, "The given invoice is invalid."),
			Self::InvoiceCreationFailed => write!(f, "Failed to create invoice."),
			Self::ChannelIdInvalid => write!(f, "The given channel ID is invalid."),
			Self::NetworkInvalid => write!(f, "The given network is invalid."),
			Self::RoutingFailed => write!(f, "Failed to find route."),
			Self::PeerInfoParseFailed => write!(f, "Failed to parse the given peer information."),
			Self::ChannelCreationFailed => write!(f, "Failed to create channel."),
			Self::ChannelClosingFailed => write!(f, "Failed to close channel."),
			Self::PersistenceFailed => write!(f, "Failed to persist data."),
			Self::WalletOperationFailed => write!(f, "Failed to conduct wallet operation."),
			Self::WalletSigningFailed => write!(f, "Failed to sign given transaction."),
			Self::TxSyncFailed => write!(f, "Failed to sync transactions."),
		}
	}
}

impl std::error::Error for Error {}

impl From<bdk::Error> for Error {
	fn from(e: bdk::Error) -> Self {
		match e {
			bdk::Error::Signer(_) => Self::WalletSigningFailed,
			_ => Self::WalletOperationFailed,
		}
	}
}

impl From<lightning_transaction_sync::TxSyncError> for Error {
	fn from(_e: lightning_transaction_sync::TxSyncError) -> Self {
		Self::TxSyncFailed
	}
}
