//
// zhttpto.rs
//
// University of Virginia - cs4414 Fall 2013
// Weilin Xu and David Evans
// Version 0.1

extern mod extra;

use extra::uv;
use extra::{net_ip, net_tcp};
use std::str;
use std::int;
use std::io;

static BACKLOG: uint = 5;
static PORT:    uint = 4414;
static IPV4_LOOPBACK: &'static str = "127.0.0.1";
static mut visitor_count: int = 0;

fn load_file(pathname : ~str) -> ~[~str] {
    let filereader : Result<@Reader, ~str> = io::file_reader(~Path(pathname));
    match filereader {
        Ok(reader) => {reader.read_lines()}
        Err(msg) => {
		println(fmt!("Oops! Error: %?", msg));
		let ret: ~[~str] = ~[];
		return ret;
	}
    }
}

fn new_connection_callback(new_conn :net_tcp::TcpNewConnection, _killch: std::comm::SharedChan<Option<extra::net_tcp::TcpErrData>>)
{
    do spawn {
        let accept_result = extra::net_tcp::accept(new_conn);
        match accept_result {
            Err(err) => {
               println(fmt!("Connection error: %?", err));
            },  
            Ok(sock) => {
                let peer_addr: ~str = net_ip::format_addr(&sock.get_peer_addr());

		let mut n = 0;
		unsafe {
			visitor_count += 1;
			println(fmt!("Visitor count: %i", visitor_count));
			n = visitor_count;
		}

                println(fmt!("Received connection from: %s", peer_addr));
                
                let read_result = net_tcp::read(&sock, 0u);
                match read_result {
                    Err(err) => {
                        println(fmt!("Receive error: %?", err));
                    },
                    Ok(bytes) => {
                        let request_str = str::from_bytes(bytes.slice(0, bytes.len() - 1));
                        println(fmt!("Request received:\n%s", request_str));
			let mut v: ~[&str] = request_str.split_iter(' ').collect();
			let mut pathName = v[1];
                        let response: ~str = ~
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=UTF-8\r\n\r\n
                             <doctype !html><html><head><title>Hello, Rust!</title>
                             <style>body { background-color: #111; color: #FFEEAA }
                                    h1 { font-size:2cm; text-align: center; color: black; text-shadow: 0 0 4mm red}
                             </style></head>
                             <body>
                             <h1>Greetings, Rusty!</h1>
			     <h1>Visitor count: </hi>" + int::to_str(n) +
                             "</body></html>\r\n<h1></h1>";

                        net_tcp::write(&sock, response.as_bytes_with_null_consume());

			pathName = pathName.slice_chars(1, pathName.len());
			println(fmt!("Pathname: %s", pathName));

			if pathName.len() > 1 {
				
				let mut file = load_file(pathName.to_owned()).concat();
				net_tcp::write(&sock, file.as_bytes_with_null_consume());
			}
                    }
                };
            }
        }
    };
}

fn main() {
    net_tcp::listen(net_ip::v4::parse_addr(IPV4_LOOPBACK), PORT, BACKLOG,
                    &uv::global_loop::get(),
                    |_chan| { println(fmt!("Listening on tcp port %u ...", PORT)); },
                    new_connection_callback);
}
