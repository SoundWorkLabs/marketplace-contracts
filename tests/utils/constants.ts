import { AnchorProvider, Provider } from "@coral-xyz/anchor";
import { PublicKey } from "@solana/web3.js";

// default provider provided by Anchor.toml
export const defaultProvider: Provider = new anchor.AnchorProvider(
	AnchorProvider.env().connection,
	AnchorProvider.env().wallet,
	AnchorProvider.defaultOptions()
);

export const SOUNDWORK_BID = new PublicKey(
	"GfK5B7Njeagu5GCeBGdVgpGzLcD8BpMDkcLeQjoXJBmY"
);
export const SOUNDWORK_LIST = new PublicKey(
	"Bh1Wa72RL4GeCPG3hKzT8W7rmvdp2sf5cbNGUsbbEMoc"
);
export const SOUNDWORK_CREATE = new PublicKey(
	"8gg4YauYXorr3YKUgZvmti61wWHDmhRfYqQZZYUnrHuc"
);
