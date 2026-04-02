mod defs;
mod network;
use defs::*;
use shared::network::{ClientMessage, Vec2};
use std::time::Duration;
use tokio::time;
use std::io::{self, Write};
async fn run_client(client_id: usize) {
    let mut pos = Vector2::new(0.0, 0.0);
    let (tx, mut rx) = network::spawn_client().await;
    let mut interval = time::interval(Duration::from_secs_f32(1.0 / 60.0));
    let mut time_alive: f32 = 0.0;
    let random_offset = (client_id as f32) * 0.5;
    loop {
        interval.tick().await; 
        while let Ok(_msg) = rx.try_recv() {}
        time_alive += 1.0 / 60.0;
        let speed = 2.0;
        let radius = 200.0;
        pos.x = ((time_alive + random_offset) * speed).cos() * radius;
        pos.y = ((time_alive + random_offset) * speed).sin() * radius;
        let _ = tx.send(ClientMessage::PositionUpdate {
            position: Vec2::new(pos.x, pos.y),
        });
        let _ = tx.send(ClientMessage::MouseUpdate {
            position: Vec2::new(pos.x, pos.y),
        });
    }
}
#[tokio::main]
async fn main() {
    print!("Enter the number of headless clients to launch: ");
    io::stdout().flush().unwrap(); 
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let num_clients: usize = input.trim().parse().unwrap_or(1);
    println!("Launching {} headless clients...", num_clients);
    let mut handles = vec![];
    for i in 0..num_clients {
        let handle = tokio::spawn(async move {
            run_client(i).await;
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.await;
    }
}
