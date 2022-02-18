// Copyright (c) 2021 Open Community Project Association https://ocpa.ch
// This software is published under the AGPLv3 license.

//! # LAN Connection Module
//! 
//! **Discover other qaul nodes on the local LAN and connect to them.**
//! 
//! This module advertises the node via mdns in the local network.
//! By default it listens to all interfaces and connects to a random port.
//! 
//! The module is configured in the configuration file:
//! 
//! ```yaml
//! [lan]
//! active = true
//! listen = "/ip4/0.0.0.0/tcp/0"
//! ```

use libp2p::{
    core::upgrade,
    dns::DnsConfig,
    noise::{NoiseConfig, X25519Spec, AuthenticKeypair},
    ping::{Ping, PingConfig, PingEvent},
    tcp::TcpConfig,
    mplex, yamux,
    mdns::{Mdns, MdnsConfig, MdnsEvent},
    Transport,
    floodsub::{Floodsub, FloodsubEvent},
    swarm::{
        Swarm, NetworkBehaviourEventProcess, ExpandedSwarm,
        protocols_handler::ProtocolsHandler,
        IntoProtocolsHandler, NetworkBehaviour
    },
    websocket::WsConfig,
    NetworkBehaviour,
};
use futures::channel::mpsc;
use prost::Message;
use std::collections::HashSet;
use log::info;
use async_std::task;
use mpsc::UnboundedReceiver;

use crate::types::QaulMessage;
use crate::node::Node;
use crate::services::{
    page,
    page::{PageMode, PageRequest, PageResponse},
    feed::{Feed, FeedMessageSendContainer},
};
use crate::storage::configuration::Configuration;
use crate::connections::{
    ConnectionModule,
    events,
};
use qaul_info::{
    QaulInfo,
    QaulInfoEvent,
};


use crate::services::feed::proto_net;

#[derive(NetworkBehaviour)]
pub struct QaulLanBehaviour {
    pub floodsub: Floodsub,
    pub mdns: Mdns,
    pub ping: Ping,
    pub qaul_info: QaulInfo,
    #[behaviour(ignore)]
    pub response_sender: mpsc::UnboundedSender<QaulMessage>,
}

pub struct Lan {
    pub swarm: ExpandedSwarm<QaulLanBehaviour, <<<QaulLanBehaviour as NetworkBehaviour>::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::InEvent, <<<QaulLanBehaviour as NetworkBehaviour>::ProtocolsHandler as IntoProtocolsHandler>::Handler as ProtocolsHandler>::OutEvent, <QaulLanBehaviour as NetworkBehaviour>::ProtocolsHandler>, 
    pub receiver: UnboundedReceiver<QaulMessage>,
}

impl Lan {
    /// Initialize swarm for LAN connection module
    pub async fn init(auth_keys: AuthenticKeypair<X25519Spec>) -> Lan {
        log::info!("Lan::init() start");

        // create a multi producer, single consumer queue
        let (response_sender, response_rcv) = mpsc::unbounded();
    
        log::info!("Lan::init() mpsc channels created");

        // TCP transport without DNS resolution on android
        // as the DNS module crashes on android due to a file system access
        #[cfg(any(target_os = "android", target_os = "ios"))]
        let transport = {
            let tcp = TcpConfig::new().nodelay(true);
            let ws_tcp = WsConfig::new(tcp.clone());
            tcp.or_transport(ws_tcp)
        };
        // create tcp transport with DNS for all other devices
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        let transport = {
            let tcp = TcpConfig::new().nodelay(true);
            let dns_tcp = DnsConfig::system(tcp).await.unwrap();
            let ws_dns_tcp = WsConfig::new(dns_tcp.clone());
            dns_tcp.or_transport(ws_dns_tcp)
        };

        log::info!("Lan::init() transport created");

        let transport_upgraded = transport
            .upgrade(upgrade::Version::V1)
            .authenticate(NoiseConfig::xx(auth_keys).into_authenticated())
            .multiplex(upgrade::SelectUpgrade::new(yamux::YamuxConfig::default(), mplex::MplexConfig::default()))
            //.timeout(std::time::Duration::from_secs(100 * 365 * 24 * 3600)) // 100 years
            .boxed();
        
        log::info!("Lan::init() transport_upgraded");

        // create ping configuration 
        // with customized parameters
        //
        // * keep connection alive
        let mut ping_config = PingConfig::new();
        ping_config = ping_config.with_keep_alive(true);

        log::info!("Lan::init() ping_config");

        let mut swarm = {
            log::info!("Lan::init() swarm creation started");

            // create MDNS behaviour
            // TODO create MdnsConfig {ttl: Duration::from_secs(300), query_interval: Duration::from_secs(30) }
            let mdns = task::block_on(Mdns::new(MdnsConfig::default())).unwrap();

            log::info!("Lan::init() swarm mdns module created");

            // TODO: set shorter re-advertisement time
            //       see here: libp2p-mdns/src/behaviour.rs
            let mut behaviour = QaulLanBehaviour {
                floodsub: Floodsub::new(Node::get_id()),
                mdns,
                ping: Ping::new(ping_config),
                qaul_info: QaulInfo::new(Node::get_id()),
                response_sender,
            };

            log::info!("Lan::init() swarm behaviour defined");

            behaviour.floodsub.subscribe(Node::get_topic());

            log::info!("Lan::init() swarm behaviour floodsub subscribed");

            Swarm::new(transport_upgraded, behaviour, Node::get_id())
        };

        log::info!("Lan::init() swarm created");
            
        // connect swarm to the listening interface in 
        // the configuration config.lan.listen
        let config = Configuration::get();
        Swarm::listen_on(
            &mut swarm,
            config.lan.listen
                .parse()
                .expect("can get a local socket"),
        )
        .expect("swarm can be started");

        log::info!("Lan::init() swarm connected");

        let lan = Lan { swarm: swarm, receiver: response_rcv };

        lan
    }
}

impl NetworkBehaviourEventProcess<QaulInfoEvent> for QaulLanBehaviour {
    fn inject_event(&mut self, event: QaulInfoEvent) {
        events::qaul_info_event( event, ConnectionModule::Lan );
    }
}

impl NetworkBehaviourEventProcess<PingEvent> for QaulLanBehaviour {
    fn inject_event(&mut self, event: PingEvent) {
        events::ping_event( event, ConnectionModule::Lan );
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for QaulLanBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(discovered_list) => {
                for (peer, _addr) in discovered_list {
                    info!("MdnsEvent::Discovered, peer {:?} to floodsub added", peer);
                    self.floodsub.add_node_to_partial_view(peer);
                }
            }
            MdnsEvent::Expired(expired_list) => {
                for (peer, _addr) in expired_list {
                    if !self.mdns.has_node(&peer) {
                        info!("MdnsEvent::Expired, peer {:?} from floodsub removed", peer);
                        self.floodsub.remove_node_from_partial_view(&peer);
                    }
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<FloodsubEvent> for QaulLanBehaviour {
    fn inject_event(&mut self, event: FloodsubEvent) {
        match event {
            FloodsubEvent::Message(msg) => {
                // feed Message
                if let Ok(resp) = proto_net::FeedContainer::decode(&msg.data[..]) {
                    Feed::received( ConnectionModule::Lan, msg.source, resp);
                }
                // Pages Messages
                else if let Ok(resp) = serde_json::from_slice::<PageResponse>(&msg.data) {
                    //if resp.receiver == node::get_id_string() {
                        info!("Response from {}", msg.source);
                        resp.data.iter().for_each(|r| info!("{:?}", r));
                    //}
                } else if let Ok(req) = serde_json::from_slice::<PageRequest>(&msg.data) {
                    match req.mode {
                        PageMode::ALL => {
                            info!("Received All req: {:?} from {:?}", req, msg.source);
                            page::respond_with_public_pages(
                                self.response_sender.clone(),
                                msg.source.to_string(),
                            );
                        }
                        PageMode::One(ref peer_id) => {
                            if peer_id.to_string() == Node::get_id_string() {
                                info!("Received req: {:?} from {:?}", req, msg.source);
                                page::respond_with_public_pages(
                                    self.response_sender.clone(),
                                    msg.source.to_string(),
                                );
                            }
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
