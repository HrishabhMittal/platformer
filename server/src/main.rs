use shared::constants::{DAMAGE_PER_SHOT, MAX_HP, PLAYER_SIZE, TICK_PER_SECOND};
use shared::network::*;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::UdpSocket;
use tokio::sync::{Mutex, mpsc};
use tokio::time::{self, Duration};

struct PlayerState {
    x: f32,
    y: f32,
    cursor_x: f32,
    cursor_y: f32,
    health: u32,
    last_seen: tokio::time::Instant,
}

struct GlobalState {
    players: HashMap<u32, PlayerState>,
}

type ClientDirectory = Arc<Mutex<HashMap<u32, SocketAddr>>>;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = 8080;
    let address = format!("{host}:{port}");
    println!("UDP Server listening on {address}");

    let socket = Arc::new(UdpSocket::bind(address).await.unwrap());

    let (tx_input, rx_input) = mpsc::channel::<ClientObject>(1024);
    let (tx_bcast, mut rx_bcast) = mpsc::channel::<ServerMessage>(1024);

    let client_directory: ClientDirectory = Arc::new(Mutex::new(HashMap::new()));

    tokio::spawn(async move {
        game_tick_loop(rx_input, tx_bcast).await;
    });

    let socket_sender = socket.clone();
    let dir_sender = client_directory.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx_bcast.recv().await {
            let encoded = bincode::serialize(&msg).unwrap();
            let mut directory = dir_sender.lock().await;

            for (_, addr) in directory.iter() {
                let _ = socket_sender.send_to(&encoded, *addr).await;
            }

            if let ServerMessage::PlayerLeft { id } = msg {
                directory.remove(&id);
            }
        }
    });

    let mut buf = [0; 1024];
    let mut next_id: u32 = 0;
    let mut addr_to_id: HashMap<SocketAddr, u32> = HashMap::new();

    loop {
        if let Ok((len, addr)) = socket.recv_from(&mut buf).await {
            let id = if let Some(&existing_id) = addr_to_id.get(&addr) {
                existing_id
            } else {
                println!("New connection from {}", addr);
                let new_id = next_id;
                next_id += 1;

                addr_to_id.insert(addr, new_id);
                client_directory.lock().await.insert(new_id, addr);

                let welcome_msg = ServerMessage::NotifyId { id: new_id };
                let encoded = bincode::serialize(&welcome_msg).unwrap();
                let _ = socket.send_to(&encoded, addr).await;

                new_id
            };

            if let Ok(client_message) = bincode::deserialize::<ClientMessage>(&buf[..len]) {
                let obj = ClientObject { id, client_message };
                let _ = tx_input.send(obj).await;
            }
        }
    }
}

async fn game_tick_loop(
    mut rx_input: mpsc::Receiver<ClientObject>,
    tx_bcast: mpsc::Sender<ServerMessage>,
) {
    let tick_duration = Duration::from_secs_f32(1.0 / TICK_PER_SECOND);
    let mut interval = time::interval(tick_duration);

    let mut state = GlobalState {
        players: HashMap::new(),
    };

    loop {
        interval.tick().await;

        while let Ok(msg) = rx_input.try_recv() {
            let events = apply_client_obj(&mut state, msg);
            for event in events {
                let _ = tx_bcast.send(event).await;
            }
        }
        let mut to_remove = Vec::new();
        for (&id, player) in &state.players {
            if player.last_seen.elapsed().as_secs() > 5 {
                let _ = tx_bcast.send(ServerMessage::PlayerLeft { id }).await;
                to_remove.push(id);
            // ONLY BROADCAST ALIVE PLAYERS
            } else if player.health > 0 { 
                let _ = tx_bcast
                    .send(ServerMessage::PlayerMoved {
                        id,
                        position: Vec2::new(player.x, player.y),
                    })
                    .await;
                let _ = tx_bcast
                    .send(ServerMessage::PlayerMouseMoved {
                        id,
                        position: Vec2::new(player.cursor_x, player.cursor_y),
                    })
                    .await;
            }
        }
        for i in to_remove {
            state.players.remove(&i);
        }
    }
}

fn apply_client_obj(state: &mut GlobalState, obj: ClientObject) -> Vec<ServerMessage> {
    let mut events = Vec::new();

    if !state.players.contains_key(&obj.id) {
        state.players.insert(
            obj.id,
            PlayerState {
                x: 0.0,
                y: 0.0,
                cursor_x: 0.0,
                cursor_y: 0.0,
                health: MAX_HP,
                last_seen: tokio::time::Instant::now(),
            },
        );
    }

    // UPDATE LAST SEEN, BUT IGNORE INPUTS IF THE PLAYER IS DEAD
    if let Some(player) = state.players.get_mut(&obj.id) {
        player.last_seen = tokio::time::Instant::now();
        if player.health == 0 {
            return events; 
        }
    }

    match obj.client_message {
        ClientMessage::PositionUpdate { position } => {
            if let Some(player) = state.players.get_mut(&obj.id) {
                player.x = position.x;
                player.y = position.y;
            }
        }
        ClientMessage::MouseUpdate { position } => {
            if let Some(player) = state.players.get_mut(&obj.id) {
                player.cursor_x = position.x;
                player.cursor_y = position.y;
            }
        }
        ClientMessage::Shoot => {
            events.push(ServerMessage::PlayerShot { id: obj.id });

            let shooter = state.players.get(&obj.id).unwrap();
            let shooter_pos = Vec2::new(shooter.x, shooter.y);
            let mouse_pos = Vec2::new(shooter.cursor_x, shooter.cursor_y);
            let shooter_id = obj.id;

            let mut hits = Vec::new();

            for (&target_id, target) in state.players.iter() {
                if target_id == shooter_id || target.health == 0 {
                    continue;
                }

                let rect = (
                    target.x - (PLAYER_SIZE / 2.0),
                    target.y - (PLAYER_SIZE / 2.0),
                    PLAYER_SIZE,
                    PLAYER_SIZE,
                );

                if shared::math::is_target_hit(shooter_pos, mouse_pos, rect) {
                    hits.push(target_id);
                }
            }

            for target_id in hits {
                if let Some(target) = state.players.get_mut(&target_id) {
                    target.health = target.health.saturating_sub(DAMAGE_PER_SHOT);
                    events.push(ServerMessage::PlayerHealthChange {
                        id: target_id,
                        health: target.health,
                    });
                }
            }
        }
    }

    events
}
