pub enum MessageType {
	Connectionless(ConnectionlessType),
	Fragmented(FragmentedHeader, ConnectionlessType)
}

pub struct FragmentedHeader {
	message_id: i32,
	total: u8,
	number: u8,
	size: i16,
	compression: Option<CompressionHeader>
}

pub struct CompressionHeader {
	size: i32,
	checksum: i32
}

pub enum ConnectionlessType {
	A2AAck,
	M2AActiveMods,
	M2AActiveMods3,
	S2CAuthChallenge1,
	S2CAuthChallenge2,
	S2CAuthComplete,
	C2SAuthConnect,
	C2SAuthRequest1,
	S2CChallenge(ChallengeFormat),
	C2MCheckMD5,
	C2SConnect(ConnectData),
	S2CConnection { challenge: Option<i32> },
	S2CConnReject { challenge: i32, reason: String },
	A2ACustom,
	A2MGetMotd,
	A2MGetServers,
	A2MGetServersBatch,
	A2MGetServersBatch2,
	A2MGetActiveMods,
	A2MGetActiveMods2,
	A2MGetActiveMods3,
	A2SGetChallenge { challenge: i32 },
	S2MGetFile,
	A2MGetMasterServers,
	A2SInfo,
	S2AInfoDetailed,
	S2AInfoSource(ServerInfo),
	M2CIsValidMD5,
	S2ALogString { data: String },
	S2ALogString2 { data: String }, // TODO: How is sv_logsecret used?
	M2AMasterServers,
	C2CMod,
	M2AMotd,
	M2MMsg,
	A2APing,
	A2SPing2,
	S2APing2Reply,
	A2SPlayer { challenge: i32 },
	S2APlayer { players: u8, data: Vec<PlayerChunk> },
	A2APrint { message: String },
	A2SRcon,
	S2CRedirect { address: i32, port: i16 },
	A2SRules { challenge: i32 },
	S2ARules { rules: i16, data: Vec<RuleChunk> },
	M2SSendFile,
	A2SServerQueryGetChallenge { challenge: i32 },
	M2AServerBatch { next_id: i32, data: Vec<ServerChunk> },
	M2AServers { data: Vec<ServerChunk> }
}

pub enum ChallengeFormat {
	Short { challenge: i32 },
	Long {
		challenge: i32,
		client_challenge: i32,
		auth_protocol: AuthProtocol,
		steamid: u64,
		secure: bool
	}
}

pub struct ConnectData {
	auth_protocol: AuthProtocol,
	challenge: i32,
	retry_challenge: i32,
	client_name: String,
	password: String
}

pub struct ServerInfo {
	protocol: u8,
	name: String,
	map: String,
	folder: String,
	game: String,
	id: i16,
	players: u8,
	maxplayers: u8,
	bots: u8,
	server_type: ServerType,
	environment: ServerEnvironment,
	protected: bool,
	anticheat: bool,
	version: String,
	port: Option<i16>,
	steamid: Option<u64>,
	sourcetv: Option<SourceTVRelayInfo>,
	keywords: Option<String>,
	gameid: Option<u64>
}

pub enum ServerType {
	Dedicated,
	NonDedicated,
	SourceTVRelay
}

pub enum ServerEnvironment {
	MacOS,
	Linux,
	Windows
}

pub struct SourceTVRelayInfo {
	port: i16,
	name: String
}

pub struct PlayerChunk {
	name: String,
	score: i32,
	duration: f32
}

pub struct RuleChunk {
	name: String,
	value: String
}

pub struct ServerChunk {
	ip: i32,
	port: i16
}

pub enum AuthProtocol {
	Steam,
	Other
}
