use std::net::{SocketAddrV4, Ipv4Addr, TcpListener, TcpStream};
use std::io::{Read, Error, Write, stdin};

fn server() -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8080);
    let listener = TcpListener::bind(socket)?;
    let port = listener.local_addr()?;

    println!("Listening on {}, access this port to end the program", port);
    let (mut tcp_stream, addr) = listener.accept()?; //block  until requested
    println!("Connection received! {:?} is sending data.", addr);
    let mut input = String::new();
    let _ = tcp_stream.read_to_string(&mut input)?;
    println!("{:?} says:\n{}", addr, input);

    Ok(())
}

fn client(msg: &str) -> Result<(), Error> {
    let loopback = Ipv4Addr::new(127, 0, 0, 1);
    let socket = SocketAddrV4::new(loopback, 8080);
    let mut stream = TcpStream::connect(socket)?;
    stream.write_all(msg.as_bytes())?;

    Ok(())
}

fn main() -> Result<(), Error> {
    println!("S or C");
    let mut raw_answer = String::new();
    let bytes_read = stdin().read_line(&mut raw_answer);
    if (bytes_read.unwrap() > 2) {
        return Ok(())
    }

    let answer = raw_answer.to_ascii_lowercase().chars().nth(0).unwrap();

    if (answer == 'c')
    {
        println!("Starting Client...");
        client("message");
    } else if (answer == 's')
    {
        println!("Starting Server...");
        server();
    } else {
        println!("'{}' type is not supported!", answer);
        return Ok(());
    }
    
    Ok(())
}