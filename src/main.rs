use std::env;
extern crate getopts;
use getopts::Options;

mod inout;
mod cards;
mod players;
mod combinations;
mod lib;

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

fn pot_distribution(game: lib::Game, mut table: &mut cards::Hand, pot: u32) {
   let qualified_players = game.qualified_players(); 

    if qualified_players.len() == 1 {
        println!("JOUEUR {} REMPORTE {}", qualified_players[0], pot);
        game.players[qualified_players[0]].cash += pot;
    } else {
        println!("*** ABATTAGE ***");

    	for j in qualified_players.iter() {
			println!("Joueur {} :", j);
			if !inout::ask_cards(&mut game.players[*j].hand, 2) { game.players[*j].state = 'f'; }
            inout::print_cards(&game.players[*j].hand);
		}

        let to_ask = table.cards_number - table.values.len();
        inout::ask_cards(&mut table, to_ask); 
		// tant qu'il y a de l'argent à distribuer on cherche des gagnants
        let mut distributed_amount = 0;
        let mut to_distribute = 0;
        let mut winners = vec![];
        let mut winners_combinations = [0; 6];

        let mut player_combination = [0; 6];
 		while distributed_amount < pot {
			winners_combinations[0] = 0;
			//nbrJoueursQualifies = nombreJoueursQualifies(pjoueur);
            //pjoueur = joueurQualifieSuivant(pjoueur);
            for j in qualified_players.iter() {
                let mut player_cards = cards::merge_hands(&table, &game.players[*j].hand);
                inout::print_cards(&player_cards);
                combinations::combination_type(&mut player_cards, &mut player_combination);
                println!("Player {}:", *j);
                inout::print_combination(&player_combination);
                if combinations::compare_combinations(&player_combination, &winners_combinations) == 1 {
                    winners = vec![*j];
                    // nouvel appel à combination_type probablement inutile…
                    combinations::combination_type(&mut player_cards, &mut winners_combinations);
                } else if combinations::compare_combinations(&player_combination, &winners_combinations) == 2 {
                    winners.push(*j);
                }
            }
            winners.sort_by_key(|k| game.players[*k].total_bet);
            let mut i = 0;
            while i < winners.len() {
                to_distribute = players::available_pot_amount(&game.players, game.players[winners[i]].number as usize) - distributed_amount;
                if to_distribute > 0 {
                    for j in i..winners.len() {
                        println!("Joueur {} REMPORTE {}", game.players[winners[j]].number, to_distribute / ((winners.len() - i) as u32));
                        game.players[winners[j]].cash += to_distribute / ((winners.len() - i) as u32);
                    }
                    if to_distribute % ((winners.len() - i) as u32) != 0 {
                        println!("Montant non divisible. Qui remporte le reste : {} ?", to_distribute % ((winners.len() - i) as u32));
                        let num = inout::ask_player_number(&game.players); 
                        game.players[num].cash += to_distribute % winners.len() as u32;
                    }
                    distributed_amount += to_distribute;
                }
                game.players[winners[i]].state = 'f';
                i+=1;
            }
		}
	}
}

fn main() {
    const N_CARDS_TO_DEAL: [usize; 4] = [0, 3, 1, 1];
    let args: Vec<String> = env::args().collect();

    let game_settings = parse_command(&args);
    let game_settings = match game_settings {
        Some(settings) => settings,
        None => return,
    };
    let mut game = lib::Game::new(game_settings.n_players, game_settings.start_cash, game_settings.first_blind);
    // auxiliary variables
    let (mut player_n, mut min_players_count): (u32, u32);
    let mut current_player: usize;
    // first hand initialisation
    println!("Bienvenue sur Monitor 0.42 !");
    println!("Vous êtes le joueur n°0, le dealer a le numéro : ");
    let mut dealer: usize = inout::ask_player_number(&game.players);
    let mut player_small_blind: usize = game.next_active_player(dealer);
    let mut player_big_blind: usize = game.next_active_player(player_small_blind);
    
    // même jeu
    loop {
        game.initialize_hand();
        // Initialisation du premier tour
        println!("*** Main numéro {} Préparation   ***", game.hand_number);
        if game.players[0].state == 'i' {
            println!("Vos cartes :");
            inout::ask_cards(&mut game.players[0].hand, 2);
            
        }
		// lorsqu'il n'y a plus que 2 joueurs en lice (« Heads-up »), le dealer est small blind
		// il est nécessaire de recalculer psmallBlind au cas où le tour précédent comptait 3 joueurs et s'est achevé par
		// la défaite du big blind.
        if game.active_players_count() == 2 {
            player_small_blind = game.next_active_player(player_big_blind);
            dealer = player_small_blind;
            current_player = dealer;
        }

        if game.players[player_small_blind].state != 'o' {
			println!("SMALL BLIND : JOUEUR {}   BET {:5}\n", player_small_blind, game.small_blind);
            game.pot += game.players[player_small_blind].make_bet(game.small_blind);
        }
        
        println!("BIG BLIND   : JOUEUR {}   BET {:5}\n", player_big_blind, game.small_blind*2);
		game.pot += game.players[player_big_blind].make_bet(game.small_blind*2);

        game.to_bet = game.small_blind*2;
        game.raise_value = game.small_blind*2;
        current_player = game.next_active_player(player_big_blind);

        // same hand
        loop{
            println!("\n*** Main numéro {:2}  Tour numéro {} ***", game.hand_number, game.round_number);
            inout::ask_cards(&mut game.table, N_CARDS_TO_DEAL[game.round_number-1]);
            
			min_players_count = game.active_players_count();
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
            game.initialize_round();
			// le premier joueur actif après le dealer commence le tour
            current_player = players::next_active_player(&players, dealer);
            if players::active_players_count(&players) <= 1 || round_n >= 5 { break; } 
        }
        pot_distribution(&mut players, &mut table, pot);
        players::reset_hand(&mut players);
        dealer = player_small_blind;
        // ne correspond pas à PokerTH: si celui qui était bigblind meurt il ne devient pas small dans PokerTH
        player_small_blind = player_big_blind;
        player_big_blind = players::next_active_player(&players, player_big_blind);
        if hand_n % game_settings.blinds_raise_interval == 0 { small_blind *= 2; }
        if players::active_players_count(&players) <= 1 { break; }
    }
    println!("Merci et à bientôt sur Monitor !");
}
