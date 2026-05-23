struct GSI {
	pub round: Round,
	pub player: Player,
	pub provider: Provider,
	pub map: Map
}
struct Round {
	pub bomb: String,
	pub phase: String,
	pub win_team: String
}
struct Player {
	pub name: String,
	pub weapons: Weapons,
	pub steamid: String,
	pub match_stats: MatchStats,
	pub xpoverload: u64,
	pub observer_slot: u64,
	pub state: State,
	pub team: String,
	pub activity: String
}
struct Weapons {
	pub weapons: Vec<Weapon>,
}
struct Weapon {
	pub name: String,
	pub ammo_clip: u64,
	pub ammo_clip_max: u64,
	pub ammo_reserve: u64,
	pub paintkit: String,
	pub state: String,
	pub r#type: String
}
struct MatchStats {
	pub mvps: u64,
	pub assists: u64,
	pub deaths: u64,
	pub score: u64,
	pub kills: u64
}
struct State {
	pub armor: u64,
	pub round_kills: u64,
	pub smoked: u64,
	pub money: u64,
	pub helmet: bool,
	pub equip_value: u64,
	pub health: u64,
	pub round_killhs: u64,
	pub defusekit: bool,
	pub flashed: u64,
	pub burning: u64
}
struct Provider {
	pub timestamp: u64,
	pub name: String,
	pub steamid: String,
	pub appid: u64,
	pub version: u64
}
struct Map {
	pub name: String,
	pub round_wins: RoundWins,
	pub mode: String,
	pub num_matches_to_win_series: u64,
	pub team_ct: TeamCt,
	pub team_t: TeamT,
	pub phase: String,
	pub round: u64
}
struct RoundWins {
	pub win_type: Vec<String>,
}
struct TeamCt {
	pub timeouts_remaining: u64,
	pub score: u64,
	pub matches_won_this_series: u64,
	pub consecutive_round_losses: u64
}
struct TeamT {
	pub matches_won_this_series: u64,
	pub score: u64,
	pub timeouts_remaining: u64,
	pub consecutive_round_losses: u64
}
