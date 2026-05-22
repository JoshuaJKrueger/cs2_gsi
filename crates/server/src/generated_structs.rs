struct GSI {
	pub player: Player,
	pub map: Map,
	pub provider: Provider,
	pub round: Round
}
struct Player {
	pub match_stats: MatchStats,
	pub team: String,
	pub observer_slot: u64,
	pub steamid: String,
	pub weapons: Weapons,
	pub state: State,
	pub name: String,
	pub activity: String
}
struct MatchStats {
	pub score: u64,
	pub assists: u64,
	pub deaths: u64,
	pub mvps: u64,
	pub kills: u64
}
struct Weapons {
	pub weapon_0: Weapon0,
	pub weapon_1: Weapon1,
	pub weapon_2: Weapon2
}
struct Weapon0 {
	pub name: String,
	pub state: String,
	pub paintkit: String,
	pub r#type: String
}
struct Weapon1 {
	pub name: String,
	pub r#type: String,
	pub state: String,
	pub paintkit: String,
	pub ammo_clip: u64,
	pub ammo_reserve: u64,
	pub ammo_clip_max: u64
}
struct Weapon2 {
	pub state: String,
	pub r#type: String,
	pub ammo_clip: u64,
	pub ammo_clip_max: u64,
	pub name: String,
	pub ammo_reserve: u64,
	pub paintkit: String
}
struct State {
	pub equip_value: u64,
	pub health: u64,
	pub flashed: u64,
	pub helmet: bool,
	pub burning: u64,
	pub round_killhs: u64,
	pub smoked: u64,
	pub round_kills: u64,
	pub armor: u64,
	pub money: u64
}
struct Map {
	pub num_matches_to_win_series: u64,
	pub phase: String,
	pub round: u64,
	pub name: String,
	pub team_ct: TeamCt,
	pub team_t: TeamT,
	pub mode: String
}
struct TeamCt {
	pub matches_won_this_series: u64,
	pub score: u64,
	pub consecutive_round_losses: u64,
	pub timeouts_remaining: u64
}
struct TeamT {
	pub score: u64,
	pub consecutive_round_losses: u64,
	pub timeouts_remaining: u64,
	pub matches_won_this_series: u64
}
struct Provider {
	pub timestamp: u64,
	pub steamid: String,
	pub name: String,
	pub appid: u64,
	pub version: u64
}
struct Round {
	pub phase: String
}
