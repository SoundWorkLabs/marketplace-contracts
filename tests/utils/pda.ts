import { PublicKey } from "@solana/web3.js";
import {
	ASSET_MANAGER_PREFIX,
	SEED_BID_DATA,
	SEED_LISTING_DATA,
	SEED_MARKETPLACE_CONFIG,
	SEED_PREFIX,
	SEED_WALLET,
	SOUNDWORK_BID_ID,
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
 * Derive the marketplace config account
 * @returns {PublicKey} listingData Address.
 */
export const findMarketplaceConfigAddress = (): PublicKey => {
	return PublicKey.findProgramAddressSync(
		[Buffer.from(SEED_PREFIX), Buffer.from(SEED_MARKETPLACE_CONFIG)],
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

/**
 * Derive the user wallet escrow address
 * @param authority user's address
 * @returns {PublicKey} listingData Address.
 */
export const findWalletAddress = (authority: PublicKey): PublicKey => {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(SEED_PREFIX),
			Buffer.from(SEED_WALLET),
			authority.toBuffer(),
		],
		SOUNDWORK_LIST_ID
	)[0];
};

/**
 * Derive the bid data account address
 *
 * @param asset asset's address
 *
 * @returns {PublicKey} The bid data Address.
 */
export const findBidDataAddress = (asset: PublicKey): PublicKey => {
	return PublicKey.findProgramAddressSync(
		[
			Buffer.from(SEED_PREFIX),
			Buffer.from(SEED_BID_DATA),
			asset.toBuffer(),
		],
		SOUNDWORK_BID_ID
	)[0];
};
