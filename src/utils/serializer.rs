use std::net::TcpStream;
use std::fmt::Debug;
use std::io::{ Read, Write, Error, ErrorKind };
use std::mem::transmute;
use std::{thread, time};
use std::io;

pub trait Reader {
    fn read_fixed_size(&mut self, bytes: &mut [u8]) -> Result<(), Error>;
}

pub trait Writer {
    fn write_fixed_size(&mut self, bytes: &[u8]) -> Result<(), Error>;
    
    fn flush(&mut self) -> Result<(), Error>;
}

pub struct Serializer{
    pub stream: TcpStream
}

impl Reader for Serializer {
    fn read_fixed_size(&mut self, buffer: &mut [u8] ) -> Result<(), Error>{
        read_exact(&mut self.stream, buffer, 10000, false)
    }
}

impl Writer for Serializer {
    fn write_fixed_size(&mut self, buffer: &[u8] ) -> Result<(), Error>{
        write_all(&mut self.stream, buffer, 10000)
    }    

    fn flush(&mut self) -> Result<(), Error>{
        // self.stream.flush()
        Ok(())
    }
}


/// The default implementation of read_exact is useless with async TcpStream as
/// it will return as soon as something has been read, regardless of
/// whether the buffer has been filled (and then errors). This implementation
/// will block until it has read exactly `len` bytes and returns them as a
/// `vec<u8>`. Except for a timeout, this implementation will never return a
/// partially filled buffer.
///
/// The timeout in milliseconds aborts the read when it's met. Note that the
/// time is not guaranteed to be exact. To support cases where we want to poll
/// instead of blocking, a `block_on_empty` boolean, when false, ensures
/// `read_exact` returns early with a `io::ErrorKind::WouldBlock` if nothing
/// has been read from the socket.
pub fn read_exact(
    conn: &mut TcpStream,
    mut buf: &mut [u8],
    timeout: u32,
    block_on_empty: bool,
) -> io::Result<()> {
    let sleep_time = time::Duration::from_millis(1);
    let mut count = 0;

    let mut read = 0;
    loop {
        match conn.read(buf) {
            Ok(0) => return Err(io::Error::new(io::ErrorKind::BrokenPipe, "disconnect")),
            Ok(n) => {
                let tmp = buf;
                buf = &mut tmp[n..];
                read += n;
            }
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                if read == 0 && !block_on_empty {
                    return Err(io::Error::new(io::ErrorKind::WouldBlock, "read_exact"));
                }
            }
            Err(e) => return Err(e),
        }
        if !buf.is_empty() {
            thread::sleep(sleep_time);
            count += 1;
        } else {
            break;
        }
        if count > timeout {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "reading from tcp stream",
            ));
        }
    }
    Ok(())
}

/// Same as `read_exact` but for writing.
pub fn write_all(conn: &mut Write, mut buf: &[u8], timeout: u32) -> io::Result<()> {
    let sleep_time = time::Duration::from_millis(1);
    let mut count = 0;

    while !buf.is_empty() {
        match conn.write(buf) {
            Ok(0) => {
                return Err(io::Error::new(
                    io::ErrorKind::WriteZero,
                    "failed to write whole buffer",
                ))
            }
            Ok(n) => buf = &buf[n..],
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => {}
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(e) => return Err(e),
        }
        if !buf.is_empty() {
            thread::sleep(sleep_time);
            count += 1;
        } else {
            break;
        }
        if count > timeout {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "reading from tcp stream",
            ));
        }
    }
    Ok(())
}








/// Trait that every type that can be serialized as binary must implement.
/// Writes directly to a Writer, a utility type thinly wrapping an
/// underlying Write implementation.
pub trait Writeable : Debug{
    /// Write the data held by this Writeable to the provided writer
    fn write(&self, writer: &mut Writer) -> Result<(), Error>;
}

/// Trait that every type that can be deserialized from binary must implement.
/// Reads directly to a Reader, a utility type thinly wrapping an
/// underlying Read implementation.
pub trait Readable
where
    Self: Sized,
{
    /// Reads the data necessary to this Readable from the provided reader
    fn read(reader: &mut Reader) -> Result<Self, Error>;
}

impl Writeable for u8{
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u8, [u8;1]>(*self) };
        writer.write_fixed_size(&bytes)
    }
}

impl Readable for u8{
    fn read(reader: &mut Reader) -> Result<u8, Error>{
        let mut bytes = [0u8;1];
        match reader.read_fixed_size(&mut bytes) {
            Ok(expr) => {
                let u_8: u8 = unsafe { transmute::<[u8;1], u8>(bytes) };
                Ok(u_8)
            },
            Err(e) => Err(e),
        }
    }
}

impl Writeable for u16{
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u16, [u8;2]>(*self) };
        writer.write_fixed_size(&bytes)
    }
}

impl Readable for u16{
    fn read(reader: &mut Reader) -> Result<u16, Error>{
        let mut bytes = [0u8;2];
        match reader.read_fixed_size(&mut bytes) {
            Ok(expr) => {
                let u_16: u16 = unsafe { transmute::<[u8;2], u16>(bytes) };
                Ok(u_16)
            },
            Err(e) => Err(e),
        }
    }
}

impl Writeable for u32{
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u32, [u8;4]>(*self) };
        writer.write_fixed_size(&bytes)
    }
}

impl Readable for u32{
    fn read(reader: &mut Reader) -> Result<u32, Error>{
        let mut bytes = [0u8;4];
        match reader.read_fixed_size(&mut bytes) {
            Ok(expr) => {
                let u_32: u32 = unsafe { transmute::<[u8;4], u32>(bytes) };
                Ok(u_32)
            },
            Err(e) => Err(e),
        }
    }
}

impl Writeable for u64{
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u64, [u8;8]>(*self) };
        writer.write_fixed_size(&bytes)
    }
}

impl Readable for u64{
    fn read(reader: &mut Reader) -> Result<u64, Error>{
        let mut bytes = [0u8;8];
        match reader.read_fixed_size(&mut bytes) {
            Ok(expr) => {
                let u_64: u64 = unsafe { transmute::<[u8;8], u64>(bytes) };
                Ok(u_64)
            },
            Err(e) => Err(e),
        }
    }
}

