pub mod game;

use game::Game;

fn run_game() {
    let mut game = Game::new();
    game.add_player("John");
    game.add_player("Jane");
    game.add_player("Richard");
    game.add_player("Amy");
    println!("{:#?}", game);
    // let run_game: bool = true;
    // while run_game {

    // };
}

fn main() {
    println!("Game Start:");
    run_game();
}
