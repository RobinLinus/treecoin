use std::net::TcpStream;
use std::fmt::Debug;
use std::io::{ Read, Write, Error, ErrorKind };
use std::mem::transmute;

pub trait Reader{
    fn read_fixed_size(&mut self, bytes: &mut [u8]) -> Result<(), Error>;
}

pub trait Writer{
    fn write_fixed_size(&mut self, bytes: &[u8]) -> Result<(), Error>;
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



impl Writeable for u32{
     fn write(&self, writer: &mut Writer) -> Result<(), Error>{
        let bytes = unsafe { transmute::<u32, [u8;4]>(*self) };
        writer.write_fixed_size(&bytes);
        Ok(())
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
        writer.write_fixed_size(&bytes);
        Ok(())
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


pub struct Serializer{
    pub stream: TcpStream
}

impl Reader for Serializer {
    fn read_fixed_size(&mut self, buffer: &mut [u8] ) -> Result<(), Error>{
        match self.stream.read(buffer) {
            Ok(0) => Err(Error::new(ErrorKind::BrokenPipe, "disconnected")),
            Ok( _ ) => Ok(()),
            Err(e) => return Err(e),
        }
    }
}

impl Writer for Serializer {
    fn write_fixed_size(&mut self, buffer: &[u8] ) -> Result<(), Error>{
        self.stream.write_all(buffer)
    }
}