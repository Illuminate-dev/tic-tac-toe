fn main() {
    let mut game = tic_tac_toe::Game::new();
    loop {
        println!("{}", game);
        game = game.player_make_move().unwrap();
    }
}
