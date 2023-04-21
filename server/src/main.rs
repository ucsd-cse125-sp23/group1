use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::str;
use slotmap::{SlotMap, SecondaryMap, DefaultKey};

mod client_components;
use client_components::*;
mod server_components;
use server_components::*;

type Entity = DefaultKey;

struct ECS {
    name_components: SlotMap<Entity, String>,
    // client components
    player_input_components: SecondaryMap<Entity, PlayerInputComponent>,
    player_weapon_components: SecondaryMap<Entity, PlayerWeaponComponent>,
    physics_components: SecondaryMap<Entity, PhysicsComponent>,
    player_camera_components: SecondaryMap<Entity, PlayerCameraComponent>,

    players: Vec<Entity>,
}

impl ECS {
    fn new() -> ECS {
        ECS {
            name_components: SlotMap::new(),
            player_input_components: SecondaryMap::new(),
            player_weapon_components: SecondaryMap::new(),
            physics_components: SecondaryMap::new(),
            player_camera_components: SecondaryMap::new(),
            players: vec![],
        }
    }
}

// #[derive(Serialize, Deserialize)]
// struct ClientData {
//     client_id: u8,
//     movement: String,
// }

// // used for any 3D value (position, velocity, acceleration)
// struct Coords {
//     x: f64,
//     y: f64,
//     z: f64,
// }

// // TODO: merge with ClientData
// struct Player {
//     position: Coords,
//     velocity: Coords,
//     hp: u8,
//     name: String,
// }

// // Dummy structure that holds movement just for now
// struct GameState {
//     players: Vec<Player>,
//     movement: String,
// }

fn handle_client(mut stream: TcpStream) {
    let mut client_buf = [0 as u8; 50]; // using 50 byte buf

    // TODO: move outside of handle client function for multiple clients
    // let mut state = GameState {
    //     players: Vec::new(),
    //     movement: String::from("none")
    // };
    // let mut dummy_player = Player {
    //     position: Coords {x:0.0, y:0.0, z:0.0},
    //     velocity: Coords {x:0.0, y:0.0, z:0.0},
    //     hp: 100,
    //     name: String::from("Dummy McDummyFace"),
    // };
    // state.players.push(dummy_player);

    while match stream.read(&mut client_buf) {
        Ok(size) => {
            // process client messages
            let message : &str = str::from_utf8(&client_buf[0..size]).unwrap();
            let mut movement = "none";
            if message.len() > 0 {
                let value : ClientData = serde_json::from_str(message).unwrap();
                println!("received: {}", value.movement);
                // update game state
                state.movement = String::from(format!("SERVER: received {}", value.movement));
            }
            // write game state back to client
            // TODO: serialize state
            let res = stream.write(state.movement.as_bytes());

            // status boolean
            size > 0
        },
        Err(_) => {
            println!("An error occurred");
            false
        }
    } {}
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("localhost:8080")?;

    // accepts connections automatically
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    /*  TODO:
            1. Update game state
            2. Send updated state
            3. Wait until tick ends

    */
    Ok(())
}
