use network::peer::{PeerChannel, PeerTracker};
use blockchain::block::Block;
use network::message::MessageHeader;
use std::io::Error;

pub enum Event{
	TooFewPeers(u32),
	IncommingPeer(PeerTracker),
	OutgoingPeer(PeerTracker),
	MessageHeader(PeerChannel),
	BlockMined(Block),
	Nothing,
} 

pub type EventResult = Result<Event, Error>;

pub trait EventSource {
	fn poll(&mut self) -> EventResult;
}

pub trait EventListener{
	
	fn poll_source<T:EventSource>(&mut self, source: &mut T) -> EventResult {
		self.on_event(source.poll()?)
	}

	fn on_event(&mut self, event: Event) -> EventResult;

}