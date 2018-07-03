use blockchain::transaction::Transaction;
use network::peer::{ PeerChannel, PeerTracker };
use blockchain::block::Block;
use network::message::MessageHeader;
use std::io;
use std::error;
use std::fmt;
use std::sync;

pub enum Event {
	TooFewPeers(u32),
	IncommingPeer(PeerTracker),
	OutgoingPeer(PeerTracker),
	MessageHeader(PeerChannel),
	BlockMined(Block),
	Transaction(Transaction),
	Nothing,
} 

pub type EventResult = Result<Event, Error>;

pub trait EventSource {
	fn poll(&mut self) -> EventResult;
}

pub trait EventListener {
	
	fn poll_source<T:EventSource>( &mut self, source: &mut T ) -> EventResult {
		self.on_event(source.poll()?)
	}

	fn on_event(&mut self, event: Event) -> EventResult;

}

#[derive(Debug)]
pub enum Error {
	Example, 
	InvalidInput,	
	StateMissMatch,
	InvalidDifficulty,
	InvalidCoinSum,
	Io( io::Error )
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Io(ref err) => write!(f, "IO error: {}", err),
            Error::Example => write!(f, "Example Error"),
            Error::InvalidInput => write!(f, "InvalidInput Error"),
            Error::StateMissMatch => write!(f, "StateMissMatch Error"),
            Error::InvalidDifficulty => write!(f, "InvalidDifficulty Error"),
            Error::InvalidCoinSum => write!(f, "InvalidCoinSum Error")
        }
    }
}

impl error::Error for Error {
	fn cause( &self ) -> Option<&error::Error> {
        match *self {
            Error::Io(ref err) => Some(err),
            Error::Example => None,
            Error::InvalidInput => None,
            _ => None
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::Io(err)
    }
}
