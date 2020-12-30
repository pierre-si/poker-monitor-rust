use std::env;
extern crate getopts;
use getopts::Options;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {}
     [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    const N_CARDS_TO_DEAL: [u8; 4] = [0, 3, 1, 1];
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("p", "players", "number of players", "10");
    opts.optopt("c", "cash", "start cash", "5000");
    opts.optopt("b", "blind", "first small blind", "10");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    println!("Bienvenue sur Monitor 0.42 !");
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    // variables constant during a game
    // le premier unwrap est nécessaire car opt_str renvoie un Optional. Il ne risque pas d'échouer étant donné que la présence de l'argument est testée par le ternaire. Le second unwrap utilise une valeur par défaut en cas d'erreur de conversion String → u32 par parse(). Le else du ternaire permet d'affecter une valeur par défaut lorsque l'argument n'est pas renseigné.
    let n_players: u32 = if matches.opt_present("p") {matches.opt_str("p").unwrap().parse().unwrap_or(10)} else {10};
    let start_cash: u32 = if matches.opt_present("c") {matches.opt_str("c").unwrap().parse().unwrap_or(5000)} else {5000};
    let first_blind: u32 = if matches.opt_present("b") {matches.opt_str("b").unwrap().parse().unwrap_or(10)} else {10};
    let blinds_raise_interval = 8;

    // variables constant during a hand
    let mut hand_n: u32;
    let mut small_blind: u32 = first_blind;

    // variables to reinitialize at the beginning of each hand
    let (mut pot, mut round): (u32, u32);

    // variables to reinitialize at the beginning of each round
    let (to_bet, raise_value): (u32, u32);
    // auxiliary variables
    let (player_n, n_min_players): (u32, u32);

    
}
