use crate::client::{Client, UserInfo};
use std::io::{Error, ErrorKind};
use std::time::Duration;
use std::{io, thread};

mod client;
mod fifobuffer;

fn main() -> io::Result<()> {
    let user = UserInfo::new(
        vec!["Fulgore_Rust".to_string()],
        "Fulgore_Rust".to_string(),
        "Fulgore".to_string()
    ).unwrap();

    let mut cli = Client::new(user);
    match cli.connect("irc.quakenet.org:6667") {
        Ok(_) => loop {
            cli.process();
            if !cli.is_connected() {
                break;
            }
            thread::sleep(Duration::from_millis(30));
        },
        Err(e) => {
            eprintln!("{e}");
            return Err(Error::from(ErrorKind::ConnectionRefused));
        }
    }
    Ok(())
    /*
    let mut test = vec![1u8,2,3,4,5,6,7,8];
    let s1: &mut [u8];
    let s2: &mut [u8];
        let raw_data = test.as_mut_ptr();
        unsafe {
            s1 = std::slice::from_raw_parts_mut(raw_data.offset(7), 1);
            s2 = std::slice::from_raw_parts_mut(raw_data, 1);
        }

    println!("{:?} {:?}", s1, s2);
    return Ok(());
    let mut stream = TcpStream::connect("irc.quakenet.org:6667").and_then(|stream| {
        stream.set_nonblocking(true)?;
        Ok(stream)
    })?;

    let mut fifo_buffer: FifoBuffer<u8> = FifoBuffer::new(128);

    let mut count = 0;
    loop {
        let n = stream.read_vectored(fifo_buffer.get_vector_for_writing().deref_mut());
        match n {
            Ok(n) => {
                fifo_buffer.wrote(n);
                count = 0;
                if n == 0 {
                    println!("Server closed...");
                    return Ok(());
                }
                let end = fifo_buffer.find_first(&[13u8, 10u8]);
                if end.is_some() {
                    let line = fifo_buffer.consume(end.unwrap());
                    if line.is_some() {
                        fifo_buffer.discard(2);
                        let text = line.unwrap().iter().map(|x| *x as char).collect::<String>();
                        println!("{text}");
                    } else {
                        println!("{end:?} {line:?}");
                    }
                }
            }
            Err(e) => {
                if e.kind() != ErrorKind::WouldBlock {
                    eprintln!("{e}");
                    return Err(e);
                } else {
                    if count > 350 {
                        println!("Closing...");
                        return Ok(());
                    }
                    count += 1;
                }
                thread::sleep(Duration::from_millis(30));
            }
        }
    }
     */
}
