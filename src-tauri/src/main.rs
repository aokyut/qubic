mod board;
mod exp;
mod tests;

use board::{
    board_action, board_back, board_init, board_last, board_next, command_run_mcts, pprint_board,
    search_mate, Agent, MuRecord, Record,
};
use proconio::input;
use std::sync::Mutex;
use tauri::{Manager, State};
// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            let record: MuRecord = Mutex::new(Record::new());

            println!("call setupu");

            app.manage(record);

            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let window = app.get_window("main").unwrap();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            board_action,
            board_next,
            board_back,
            board_init,
            board_last,
            search_mate,
            command_run_mcts
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// fn main() {
//     let player = Agent::Human;
//     let mut b = board::Board::new();
//     loop {
//         if b.is_win() | b.is_draw() {
//             break;
//         }
//         let scores = board::Node::new(b.clone()).search(50, 100000);
//         pprint_board(&b);
//         println!("{:#?}", scores);
//         let mut node = board::Node::new(b.clone());
//         let action = player.get_action(&b);
//         b = b.next(action);
//     }
// }

fn rate_analysis() {
    let agents = vec![
        Agent::Random,
        Agent::Mcts(50, 100),
        Agent::Mcts(50, 200),
        Agent::Mcts(50, 300),
        Agent::Mcts(50, 400),
        Agent::Mcts(50, 500),
        Agent::Mcts(50, 600),
        Agent::Mcts(50, 700),
        Agent::Mcts(50, 800),
        Agent::Mcts(50, 900),
        Agent::Mcts(50, 1000),
        Agent::Mcts(50, 1100),
        Agent::Mcts(50, 1200),
        Agent::Mcts(50, 1300),
        Agent::Mcts(50, 1400),
        Agent::Mcts(50, 1500),
        Agent::Mcts(50, 1600),
        Agent::Mcts(50, 1700),
        Agent::Mcts(50, 1800),
        Agent::Mcts(50, 1900),
        Agent::Mcts(50, 2000),
    ];

    let rates = vec![
        1500.0, 1701.7267, 2146.0044, 2262.773, 2335.1033, 1889.678, 2110.4883, 2147.6833,
        2150.5618, 2316.7488, 2328.4158, 2277.1406,
        // 1500.0,
        // 1500.0,
    ];

    let mut ratings = exp::Rating::new(agents);
    ratings.temp = 10.0;
    ratings.playn(10000);
    ratings.temp = 1.0;
    ratings.playn(4000);
    ratings.temp = 1.0 / 16.0;
    ratings.playn(4000);
    ratings.temp = 1.0 / 160.0;
    ratings.playn(4000);
    ratings.temp = 1.0 / 1600.0;
    ratings.playn(4000);
    ratings.print();
    println!("temp:{}", ratings.temp);
}
