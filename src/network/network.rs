
use std::io::Error;
use std::io::Write;
use std::collections::HashMap;
use std::net::SocketAddr;

use std::net::{TcpListener, TcpStream};
use std::{io, thread};
use std::time::Duration;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, RwLock};

type Buffer = [u8];

pub struct Network {
    peers: Peers,
    stop: Arc<AtomicBool>,
}

impl Network {
    
    pub fn new()->Network{
        Network{
            peers: Peers::new(),
            stop: Arc::from(AtomicBool::from(true))
        }
    }

    fn listen(&self) -> Result<(),Error>{
        let addrs = [
            SocketAddr::from(([127, 0, 0, 1], 7000)),
            SocketAddr::from(([127, 0, 0, 1], 7001)),
        ];
        
        let listener = TcpListener::bind(&addrs[..])?;
        listener.set_nonblocking(true)?;

        let sleep_time = Duration::from_millis(1);
        
        loop{
            match listener.accept() {
                Ok((stream, _peer_addr)) => {
                    self.handle_incomming(stream);
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // nothing to do, will retry in next iteration
                }
                Err(_e) => {
                    // println!("Couldn't establish new client connection: {:?}", e);
                }
            }
            if self.stop.load(Ordering::Relaxed) {
                break;
            }
            thread::sleep(sleep_time);
        }
        // thread::spawn(||loop{
            
        // })
        Ok(())
    }

    fn handle_incomming(&self, stream: TcpStream){
        self.check_banned(&stream);
        self.handle_new_peer(stream);
    }

    fn check_banned(&self, stream: &TcpStream){
        // Todo: check list
    }

    fn handle_new_peer(&self, mut stream :TcpStream)-> Result<(),Error>{
        let peer = Peer::accept(
            &mut stream,
        );
        Ok(())
    }

    pub fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

struct Peer {
    connection: Option<conn::Tracker>
}

impl Peer {

    fn new()->Peer{
        unimplemented!()
    }

    pub fn accept(conn: &mut TcpStream) -> Peer{
        unimplemented!()
    }

    fn start(){
        self.connection = Some(conn::listen(conn, handler));
    }

    fn send(&mut self, buffer:&Buffer){
        self.connection.write_all(buffer);
    }

    fn eventLoop(){
        thread::spawn(||loop {
            unimplemented!();
        });
    }
}


struct Peers {
    peers_set: RwLock<HashMap<SocketAddr, Arc<RwLock<Peer>>>>
}

impl Peers {
    fn new() -> Peers {
        Peers{
            peers_set: RwLock::new(HashMap::new())
        }
    }
}
