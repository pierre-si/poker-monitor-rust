use std::env;
extern crate getopts;
use getopts::Options;

mod inout;
mod cards;
mod players;

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
    const N_CARDS_TO_DEAL: [u8; 4] = [0, 3, 1, 1];
    let args: Vec<String> = env::args().collect();

    let game_settings = parse_command(&args);
    let game_settings = match game_settings {
        Some(settings) => settings,
        None => return,
    };
   // variables constant during a hand
    let mut hand_n: u32 = 0;
    let mut small_blind: u32 = game_settings.first_blind;
    // variables to reinitialize at the beginning of each hand
    let (mut pot, mut round): (u32, u32);
    let mut table = cards::Deck::create_deck(5);
    // variables to reinitialize at the beginning of each round
    let (mut to_bet, mut raise_value): (u32, u32);
    // auxiliary variables
    let (mut player_n, mut n_min_players): (u32, u32);

    // setup
    let mut players = players::create_players(game_settings.n_players, game_settings.start_cash);
    // first hand initialisation
    println!("Bienvenue sur Monitor 0.42 !");
    println!("Vous êtes le joueur n°0, le dealer a le numéro : ");
    let dealer: usize = inout::ask_player_number(&players);
    let player_small_blind: usize = players::next_active_player(&players, dealer);
    let player_big_blind: usize = players::next_active_player(&players, player_small_blind);

    // même jeu
    loop {
        // préparation de la main
        hand_n += 1;
        // Réinitialisation des variables de la main
        pot = 0;
        round = 1;
        table.reset_cards();
        println!("{:?}", table);
        // Initialisation du premier tour
        println!("*** Main numéro {} Préparation   ***\n", hand_n);
        if players[0].state == 'i' {
            println!("Vos cartes :");
            inout::ask_cards(&mut players[0].deck, 2);
            
        }
        
        if true {break;}
    }
}
