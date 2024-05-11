import { AnchorProvider, Provider } from "@coral-xyz/anchor";
import { Connection, PublicKey } from "@solana/web3.js";
import dotenv from "dotenv";

dotenv.config();

const connection = new Connection(
	`https://devnet.helius-rpc.com/?api-key=${process.env.HELIUS_API_KEY}`
);

// default provider provided by Anchor.toml
export const defaultProvider: Provider = new AnchorProvider(
	connection,
	AnchorProvider.env().wallet,
	AnchorProvider.defaultOptions()
);

export const SOUNDWORK_BID_ID = new PublicKey(
	"7Kehs8uKmqUq62s9aXTtU348HagoeR8RLZiUY5XMsct3"
);
export const SOUNDWORK_CREATE_ID = new PublicKey(
	"DEmW5Gz7c4PzaMXayyYjWkkDfiXeEQoLysSdgCuepw5b"
);
export const SOUNDWORK_LIST_ID = new PublicKey(
	"Cdn2CtPiYR9Lar4JnzhQbY3Gy4s6xYVjQLy3NBvZAN6k"
);

// external programs
export const CORE_PROGRAM_ID = new PublicKey(
	"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);

// ------------------------------------------- seeds
export const SEED_PREFIX = "Kessoku";

export const ASSET_MANAGER_PREFIX = "Seika";

export const SEED_LISTING_DATA = "Hitori";
