

fn main(){

    // get your ip 
    // ifconfig
    let mut pkt_buf = [0u8; 1500];
    
    let pkt = packet_builder!(
        pkt_buf,
        ether({set_destination => MacAddr(0x90:0x9a:0x4a:0x3a:0x83:0xfe), set_source => MacAddr(0,0,0,0,0,1)}) / 
        ipv4({set_source => ipv4addr!("127.0.0.1"), set_destination => ipv4addr!("11.23.58.13") }) /
        icmp_echo_req({set_icmp_type => IcmpTypes::EchoRequest}) / 
        payload({"hello".to_string().into_bytes()})
    );

    sender.send_to(pkt.packet(), None).unwrap().unwrap();
    

}
