struct GSI {
	pub provider: Provider,
	pub round: Round,
	pub player: Player,
	pub map: Map
}
struct Provider {
	pub timestamp: u64,
	pub appid: u64,
	pub name: String,
	pub steamid: String,
	pub version: u64
}
struct Round {
	pub win_team: String,
	pub bomb: String,
	pub phase: String
}
struct Player {
	pub weapons: Weapons,
	pub xpoverload: u64,
	pub steamid: String,
	pub activity: String,
	pub name: String,
	pub state: State,
	pub match_stats: MatchStats,
	pub observer_slot: u64,
	pub team: String
}
struct Weapons {
	pub weapon_3: Weapon3,
	pub weapon_4: Weapon4,
	pub weapon_6: Weapon6,
	pub weapon_0: Weapon0,
	pub weapon_7: Weapon7,
	pub weapon_2: Weapon2,
	pub weapon_1: Weapon1,
	pub weapon_5: Weapon5
}
struct Weapon3 {
	pub paintkit: String,
	pub r#type: String,
	pub state: String,
	pub ammo_reserve: u64,
	pub ammo_clip_max: u64,
	pub name: String,
	pub ammo_clip: u64
}
struct Weapon4 {
	pub paintkit: String,
	pub ammo_clip: u64,
	pub state: String,
	pub r#type: String,
	pub ammo_clip_max: u64,
	pub name: String,
	pub ammo_reserve: u64
}
struct Weapon6 {
	pub state: String,
	pub r#type: String,
	pub ammo_clip: u64,
	pub paintkit: String,
	pub ammo_clip_max: u64,
	pub ammo_reserve: u64,
	pub name: String
}
struct Weapon0 {
	pub ammo_clip_max: u64,
	pub r#type: String,
	pub state: String,
	pub ammo_clip: u64,
	pub name: String,
	pub paintkit: String,
	pub ammo_reserve: u64
}
struct Weapon7 {
	pub state: String,
	pub paintkit: String,
	pub ammo_reserve: u64,
	pub name: String,
	pub r#type: String
}
struct Weapon2 {
	pub ammo_clip_max: u64,
	pub ammo_reserve: u64,
	pub ammo_clip: u64,
	pub paintkit: String,
	pub name: String,
	pub r#type: String,
	pub state: String
}
struct Weapon1 {
	pub name: String,
	pub ammo_clip: u64,
	pub ammo_clip_max: u64,
	pub ammo_reserve: u64,
	pub paintkit: String,
	pub state: String,
	pub r#type: String
}
struct Weapon5 {
	pub paintkit: String,
	pub r#type: String,
	pub ammo_clip_max: u64,
	pub name: String,
	pub ammo_clip: u64,
	pub state: String,
	pub ammo_reserve: u64
}
struct State {
	pub defusekit: bool,
	pub helmet: bool,
	pub round_kills: u64,
	pub health: u64,
	pub money: u64,
	pub round_killhs: u64,
	pub smoked: u64,
	pub equip_value: u64,
	pub burning: u64,
	pub armor: u64,
	pub flashed: u64
}
struct MatchStats {
	pub mvps: u64,
	pub assists: u64,
	pub deaths: u64,
	pub kills: u64,
	pub score: u64
}
struct Map {
	pub phase: String,
	pub team_t: TeamT,
	pub name: String,
	pub round_wins: RoundWins,
	pub round: u64,
	pub mode: String,
	pub num_matches_to_win_series: u64,
	pub team_ct: TeamCt
}
struct TeamT {
	pub score: u64,
	pub consecutive_round_losses: u64,
	pub matches_won_this_series: u64,
	pub timeouts_remaining: u64
}
struct RoundWins {
	pub win_type: Vec<String>,
}
struct TeamCt {
	pub matches_won_this_series: u64,
	pub score: u64,
	pub consecutive_round_losses: u64,
	pub timeouts_remaining: u64
}
