use oping::{Ping, PingResult};
fn main() -> u8{
    println!("Hello, world!");

    let mut ping = Ping::new();
    
    ping.set_timeout(5.0);  // timeout of 5.0 seconds
    ping.add_host("127.0.0.1");  // fails here if socket can't be created
    // try!(ping.add_host("localhost"));  // fails here if socket can't be created
    // try!(ping.add_host("other_host"));
    // try!(ping.add_host("::1"));  // IPv4 / IPv6 addresses OK
    // try!(ping.add_host("1.2.3.4"));
    let responses = ping.send();
    for resp in responses {
        if resp.dropped > 0 {
            println!("No response from host: {}", resp.hostname);
        } else {
            println!("Response from host {} (address {}): latency {} ms",
                resp.hostname, resp.address, resp.latency_ms);
            println!("    all details: {:?}", resp);
        }
    }
    return 1
}
