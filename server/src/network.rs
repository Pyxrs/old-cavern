use std::{
    net::{self, SocketAddr},
    sync::mpsc::{self, Receiver, Sender},
};

use shared::{
    log::debug,
    network::{
        server::{self, Server},
        SendMode,
    },
    packets::Packet,
    Ignore, Module,
};

struct Network {
    server: Server,
    inbound_packets: Sender<(SocketAddr, Packet)>,
    outbound_packets: Receiver<(SocketAddr, Packet)>,
}

impl<A: net::ToSocketAddrs>
    Module<A, (Sender<(SocketAddr, Packet)>, Receiver<(SocketAddr, Packet)>), ()> for Network
{
    #[profiling::function]
    fn new(
        address: A,
    ) -> (
        (Sender<(SocketAddr, Packet)>, Receiver<(SocketAddr, Packet)>),
        Self,
    ) {
        let config = Default::default();

        // Create a server object
        let server = Server::bind(address, config).unwrap();
        let outbound_packets = mpsc::channel();
        let inbound_packets = mpsc::channel();

        (
            (outbound_packets.0, inbound_packets.1),
            Self {
                server,
                inbound_packets: inbound_packets.0,
                outbound_packets: outbound_packets.1,
            },
        )
    }

    #[profiling::function]
    fn run(mut self, _: ()) {
        loop {
            // Process inbound UDP frames and handle events
            for event in self.server.step() {
                match event {
                    server::Event::Connect(client_address) => {
                        debug!("[{:?}] connected", client_address);
                    }
                    server::Event::Disconnect(client_address) => {
                        debug!("[{:?}] disconnected", client_address);
                    }
                    server::Event::Error(client_address, err) => {
                        debug!("[{:?}] error: {:?}", client_address, err);
                    }
                    server::Event::Receive(client_address, packet_data) => {
                        let (id_bytes, data) = packet_data.split_at(4);
                        let id = u32::from_be_bytes(id_bytes.try_into().unwrap_or_default());

                        self.inbound_packets
                            .send((client_address, Packet::deserialize(id, data).unwrap()))
                            .ignore();

                        debug!("[{:?}] received \"{:?}\"", client_address, packet_data);
                    }
                }
            }

            for (address, packet) in self.outbound_packets.try_iter() {
                self.server.client(&address).unwrap().borrow_mut().send(
                    packet.serialize().into(),
                    0,
                    SendMode::Reliable,
                )
            }

            // Flush outbound UDP frames
            self.server.flush();

            std::thread::sleep(std::time::Duration::from_millis(30));
        }
    }
}
