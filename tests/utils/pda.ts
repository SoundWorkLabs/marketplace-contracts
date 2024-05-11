import { PublicKey } from "@solana/web3.js";
import {
	ASSET_MANAGER_PREFIX,
	SEED_LISTING_DATA,
	SEED_PREFIX,
	SOUNDWORK_LIST_ID,
} from "./constants";

/**
 * Derive the asset manager account address
 * @returns {PublicKey} The asset Manager Address.
 */
export const findAssetManagerAddress = (): PublicKey => {
	return PublicKey.findProgramAddressSync(
		[Buffer.from(SEED_PREFIX), Buffer.from(ASSET_MANAGER_PREFIX)],
		SOUNDWORK_LIST_ID
	)[0];
};

/**
 * Derive the listing data account address
 * @param asset Asset address
 * @returns {PublicKey} listingData Address.
 */
export const findListingDataAddress = (asset: PublicKey): PublicKey => {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(SEED_PREFIX),
			Buffer.from(SEED_LISTING_DATA),
			asset.toBuffer(),
		],
		SOUNDWORK_LIST_ID
	)[0];
};
