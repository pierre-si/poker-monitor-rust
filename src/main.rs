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
    const N_CARDS_TO_DEAL: [usize; 4] = [0, 3, 1, 1];
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
    let (mut pot, mut round_n): (u32, usize);
    let mut table = cards::Deck::new(5);
    // variables to reinitialize at the beginning of each round
    let (mut to_bet, mut raise_value): (u32, u32);
    // auxiliary variables
    let (mut player_n, mut min_players_count): (u32, u32);
    let mut current_player: usize;

    // setup
    let mut players = players::create_players(game_settings.n_players, game_settings.start_cash);
    // first hand initialisation
    println!("Bienvenue sur Monitor 0.42 !");
    println!("Vous êtes le joueur n°0, le dealer a le numéro : ");
    let mut dealer: usize = inout::ask_player_number(&players);
    let mut player_small_blind: usize = players::next_active_player(&players, dealer);
    let mut player_big_blind: usize = players::next_active_player(&players, player_small_blind);

    // même jeu
    loop {
        // préparation de la main
        hand_n += 1;
        // Réinitialisation des variables de la main
        pot = 0;
        round_n = 1;
        table.reset_cards();
        println!("{:?}", table);
        // Initialisation du premier tour
        println!("*** Main numéro {} Préparation   ***\n", hand_n);
        if players[0].state == 'i' {
            println!("Vos cartes :");
            inout::ask_cards(&mut players[0].deck, 2);
            
        }
		// lorsqu'il n'y a plus que 2 joueurs en lice (« Heads-up »), le dealer est small blind
		// il est nécessaire de recalculer psmallBlind au cas où le tour précédent comptait 3 joueurs et s'est achevé par
		// la défaite du big blind.
        if players::active_players_count(&players) == 2 {
            player_small_blind = players::next_active_player(&players, player_big_blind);
            dealer = player_small_blind;
            current_player = dealer;
        }

        if players[player_small_blind].state != 'o' {
			println!("SMALL BLIND : JOUEUR {}   BET {:5}\n", player_small_blind, small_blind);
            pot += players[player_small_blind].make_bet(small_blind);
        }
        
        println!("BIG BLIND   : JOUEUR {}   BET {:5}\n", player_big_blind, small_blind*2);
		pot += players[player_big_blind].make_bet(small_blind*2);

        to_bet = small_blind*2;
        raise_value = small_blind*2;
        current_player = players::next_active_player(&players, player_big_blind);

        // same hand
        loop{
            println!("\n*** Main numéro {:2}  Tour numéro {} ***", hand_n, round_n);
            inout::ask_cards(&mut table, N_CARDS_TO_DEAL[round_n-1]);
            
			min_players_count = players::active_players_count(&players);
            player_n = 0;
            // same round
			loop{
                println!("\nPOT  {:5}  REQUIS {:5}  RAISE {:5}", pot, to_bet, raise_value);
                let action = inout::ask_action(&players, current_player, to_bet, raise_value);
                players[current_player].action(action, &mut pot, &mut to_bet, &mut raise_value);
                current_player = players::next_active_player(&players, current_player);
                player_n += 1;
                // les joueurs actifs au début du tour doivent jouer au moins une fois
                if players[current_player].state != 'i' || (player_n >= min_players_count && players[current_player].round_bet == to_bet) {
                    break;
                }
			}
			// Préparation du tour suivant //
            round_n += 1;
			to_bet = 0;
            raise_value = small_blind*2;
            players::reset_round(&mut players);
			// le premier joueur actif après le dealer commence le tour
            current_player = players::next_active_player(&players, dealer);
            if players::active_players_count(&players) <= 1 || round_n >= 5 { break; } 
		}
        if true {break;}
    }
}
