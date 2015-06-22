use server::{RpgServer, WorldState, ServerStatus, Player};

use shared::net::send_packet;
use shared::packets::Packet;

use std::thread;
use std::net::{Shutdown, TcpStream};

#[test]
fn test_server_connection() {
    let mut server = RpgServer::new("127.0.0.1:0").unwrap();
    let addr = server.local_addr().unwrap();
    server.start();

    assert!(server.status() == ServerStatus::Running{
        world_running: true,
        server_running: true,
        socket_running: true,
    });


    {
        let arc_state = server.get_state();
        let state = arc_state.read().unwrap();
        let players = state.get_players();
        assert!(players.len() == 0);
    }

    let mut client = TcpStream::connect(addr).unwrap();

    thread::sleep_ms(100); // This is a big ugly, but no choice really
                           // It does give a QA test, it shouldn't take
                           // Too long for a client to be connected.
                           //
    {
        let arc_state = server.get_state();
        let state = arc_state.read().unwrap();
        let players = state.get_players();
        assert!(players.len() == 1);
    }


    send_packet(&mut client, &Packet::AuthPlayer("Neikos".to_string()));

    thread::sleep_ms(100);


    {
        let arc_state = server.get_state();
        let state = arc_state.read().unwrap();
        let players = state.get_players();
        assert!(players.len() == 1);

        for (id, ply) in players.iter() {
            // There should be only one dude!
            if let &Some(ref name) = ply.get_name() {
                assert!(name == "Neikos");
            }
        }
    }

    client.shutdown(Shutdown::Both);

    thread::sleep_ms(100);

    {
        let arc_state = server.get_state();
        let state = arc_state.read().unwrap();
        let players = state.get_players();
        assert!(players.len() == 0);
    }
}
