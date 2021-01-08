use std::env;
extern crate getopts;
use getopts::Options;

use monitor::Game;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}
     [options]", program);
    print!("{}", opts.usage(&brief));
}

struct GameSettings {
    n_players: u32,
    start_cash: u32,
    first_blind: u32,
    blinds_raise_interval: u32,
}

fn parse_command(args: &[String]) -> Option<GameSettings> {
    let program = args[0].clone();
    // paramétrage du jeu
    let mut opts = Options::new();
    opts.optopt("p", "players", "number of players", "10");
    opts.optopt("c", "cash", "start cash", "5000");
    opts.optopt("b", "blind", "first small blind", "10");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return None;
    }
    // variables constant during a game
    // le premier unwrap est nécessaire car opt_str renvoie un Optional. Il ne risque pas d'échouer étant donné que la présence de l'argument est testée par le ternaire. Le second unwrap utilise une valeur par défaut en cas d'erreur de conversion String → u32 par parse(). Le else du ternaire permet d'affecter une valeur par défaut lorsque l'argument n'est pas renseigné.
    // eventuellement utiliser .expect() après le .parse() pour traiter les cas limite.
    let n_players: u32 = if matches.opt_present("p") {matches.opt_str("p").unwrap().parse().unwrap_or(10)} else {10};
    let start_cash: u32 = if matches.opt_present("c") {matches.opt_str("c").unwrap().parse().unwrap_or(5000)} else {5000};
    let first_blind: u32 = if matches.opt_present("b") {matches.opt_str("b").unwrap().parse().unwrap_or(10)} else {10};
    let blinds_raise_interval = 8;

    Some(GameSettings { n_players, start_cash, first_blind, blinds_raise_interval})
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let game_settings = parse_command(&args);
    let game_settings = match game_settings {
        Some(settings) => settings,
        None => return,
    };
    println!("Bienvenue sur Monitor 0.42 !");
    let mut game = Game::new(game_settings.n_players, game_settings.start_cash, game_settings.first_blind, game_settings.blinds_raise_interval);
    game.run(); 
    println!("Merci et à bientôt sur Monitor !");
}
