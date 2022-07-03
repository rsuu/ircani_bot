use std::{
    io::{BufRead, BufReader, BufWriter, Read, Write},
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr, TcpStream, ToSocketAddrs},
};

use regex::Regex;

fn main() {
    let irc = "irc.ea.libera.chat:6667";
    let stream = TcpStream::connect(irc).unwrap();
    println!("{}", irc);

    stream.set_read_timeout(None);
    stream.set_write_timeout(None);

    let mut reader = BufReader::new(stream.try_clone().unwrap());
    let mut writer = BufWriter::new(stream.try_clone().unwrap());

    writer.write("NICK rustbot\r\n".as_bytes()).unwrap();
    writer
        .write("USER rustbot 8 * :rustbot\r\n".as_bytes())
        .unwrap();
    writer.flush().unwrap();

    let mut is_connected = false;
    let mut is_exit = false;

    let mut response = String::new();
    let mut send_to: String = "".to_string();

    let re: Regex = Regex::new(r"^.*send to (?P<name>.*).*$").unwrap();

    while !is_exit {
        reader.read_line(&mut response).unwrap();

        match response.as_str() {
            s if !s.is_empty() => {
                if s.to_lowercase().contains("send to") {
                    is_connected = true;
                    //println!("s = {}", s.trim_end());
                    send_to = re
                        .captures(s.trim_end())
                        .and_then(|cap| cap.name("name").map(|f| f.as_str().to_string()))
                        .unwrap();
                } else if s.to_lowercase().contains("ping") {
                    if send_to.is_empty() {
                        println!("None");
                    } else {
                        send_msg(&mut writer, send_to.as_str(), "PING");
                    }
                } else if s.to_lowercase().contains("hello") {
                    send_msg(&mut writer, send_to.as_str(), "HELLO");
                } else if s.to_lowercase().contains("bye") {
                    send_msg(&mut writer, send_to.as_str(), "BYE");

                    is_exit = true;

                    println!("End of Stream!");
                } else {
                    //println!("s: {}", s);
                }
            }

            _ => {}
        }

        response.clear();
    }

    println!("Connection timed out!");
}

fn send_msg<T>(w: &mut BufWriter<TcpStream>, user: T, msg: T)
where
    T: AsRef<str> + std::fmt::Display,
{
    let send = format!("PRIVMSG {} :{}\r\n", user, msg);

    w.write(send.as_bytes()).unwrap();
    w.flush().unwrap();

    // println!("DONE: {}",msg);
}

// REF
// https://github.com/kzzch/rustbot/blob/master/rustdrop.rs
// Default  | irc.libera.chat
// Europe  | irc.eu.libera.chat
// US & Canada  | irc.us.libera.chat
// Australia and New Zealand  | irc.au.libera.chat
// East Asia  | irc.ea.libera.chat
// IPv4 only  | irc.ipv4.libera.chat
// IPv6 only  | irc.ipv6.libera.chat
//
// Additional ports are available:
// Plain-text  | 6665-6667, 8000-8002
// TLS  | 6697, 7000, 7070
