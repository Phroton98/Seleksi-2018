// File block 

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
	pub height : String,
	pub time_stamp : String,
	pub transactions : String,
	pub internal_transactions : String,
	pub hash : String,
	pub parent_hash : String,
	pub sha3uncles : String,
	pub mined_by : String,
	pub difficulty : String,
	pub total_difficulty : String,
	pub size : String,
	pub gas_used : String,
	pub gas_limit : String,
	pub nonce : String,
	pub block_reward : String,
	pub uncles_reward : String,
}