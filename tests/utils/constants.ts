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

// ------------------------------------------- programs

export const SOUNDWORK_BID_ID = new PublicKey(
	"4mFDYND4AVREYEJXCPhjq1LnbjELHHebJqG3NZechA7X"
);
export const SOUNDWORK_CREATE_ID = new PublicKey(
	"4iraDthfMHkgrvWsLz4mfCyHJY4JKc31TTxGMZKrc6r8"
);
export const SOUNDWORK_LIST_ID = new PublicKey(
	"EA4ptgF3TYjDBGYJApAoZoyCbCYw6P5mGU5noCe1Z97"
);

// external programs
export const CORE_PROGRAM_ID = new PublicKey(
	"CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d"
);

// ------------------------------------------- seeds
export const SEED_PREFIX = "Kessoku";

export const ASSET_MANAGER_PREFIX = "Seika";

export const SEED_LISTING_DATA = "Hitori";

export const SEED_MARKETPLACE_CONFIG = "Ijichi";

export const SEED_WALLET = "Yamada";

export const SEED_BID_DATA = "Futari";

// --------------------------------------------------- accounts
export const PAYMENT_MINT = new PublicKey(
	"Gh9ZwEmdLJ8DscKNTkTqPbNwLNNBjuSzaG9Vp2KGtKJr"
); // USDC - dev

// ------------------------------------------------------- testing
export let asset = new PublicKey("");
