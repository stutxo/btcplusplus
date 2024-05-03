use ldk_node::bitcoin::secp256k1::PublicKey;

use ldk_node::bitcoin::Network;
use ldk_node::io::sqlite_store::SqliteStore;
use ldk_node::lightning::ln::msgs::SocketAddress;
use ldk_node::lightning_invoice::Bolt11Invoice;
use ldk_node::Builder;
use ldk_node::Config;
use ldk_node::Node;
use std::io;
use std::io::Write;
use std::str::FromStr;

pub(crate) fn poll_for_user_input(node: &Node) {
    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // Without flushing, the `>` doesn't print
        let mut line = String::new();
        if let Err(e) = io::stdin().read_line(&mut line) {
            break println!("ERROR: {}", e);
        }

        if line.len() == 0 {
            // We hit EOF / Ctrl-D
            break;
        }

        let mut words = line.split_whitespace();
        let a = words.next();
        println!("Command: {:?}", format!("this is so weird {:?}", a));
    }
}

#[tokio::main]
async fn main() {
    let mut config = Config::default();
    config.listening_addresses = Some(vec![
        SocketAddress::from_str(&format!("0.0.0.0:{}", 9735)).expect("AAA")
    ]);
    let mut builder = Builder::from_config(config);

    builder.set_network(Network::Bitcoin);
    builder.set_esplora_server("https://blockstream.info/api".to_string());
    builder.set_gossip_source_rgs("https://rapidsync.lightningdevkit.org/snapshot".to_string());

    let blocking_task = tokio::task::spawn_blocking(move || {
        let node = builder.build().unwrap();

        node.start().unwrap();

        // let funding_address = node.onchain_payment().send_to_address(address, amount_sats);

        println!("Node started {}", node.node_id());

        poll_for_user_input(&node);

        // let node_id = PublicKey::from_str("NODE_ID").unwrap();
        // let node_addr = SocketAddress::from_str("IP_ADDR:PORT").unwrap();
        // node.connect_open_channel(node_id, node_addr, 10000, None, None, false)
        //     .unwrap();

        let event = node.wait_next_event();
        println!("EVENT: {:?}", event);
        node.event_handled();

        //listen for channel open
        //

        // let invoice = Bolt11Invoice::from_str("INVOICE_STR").unwrap();
        // node.send_payment(&invoice).unwrap();
    });

    blocking_task.await.unwrap();
}
