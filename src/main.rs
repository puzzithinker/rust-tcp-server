//
//@Simon Leung
//@A Simple Rust TCP Server 
//

//import necessary libraries
use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time;


fn handle_client(mut stream: TcpStream) -> Result<(), Error>{
    //function for handling input from client side (TcpStream)
    let mut buf = [0; 512];
    //create a mutable array called buf, with value=0 and length=512
    for _ in 0..1000 {
        let bytes_read = stream.read(&mut buf)?;
        //Copy the content of TCP Stream to buf
        if bytes_read == 0 {
            return Ok(());
            //if bytes read = 0 then return ok(end)
        }

        stream.write(&buf[..bytes_read])?;
        //echo the string back to the stream
        thread::sleep(time::Duration::from_secs(1 as u64));
        //sleep for 1s
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    //define the listener and bind the localhost and port 8080
    let mut thread_vec: Vec<thread::JoinHandle<()>> = Vec::new();
    //Create a container (Vector) for thread join handle
    
    for stream in listener.incoming() {
        let stream = stream.expect("failed!");
        //define the stream and prompt error message. Otherwise, continue the flow below
        let handle = thread::spawn(move || {
            handle_client(stream)
        .unwrap_or_else(|error| eprintln!("{:?}", error));
        });
        //create thread for each input stream
        thread_vec.push(handle);
        //add the handle into the container
    }

    for handle in thread_vec {
        //the for loop is for awaiting the termination of the thread
        handle.join().unwrap();
        //wait for the join handle
    }

    Ok(())
}