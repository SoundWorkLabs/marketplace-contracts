import { AnchorProvider, setProvider } from "@coral-xyz/anchor";

import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import { BN } from "bn.js";
import { CORE_PROGRAM_ID } from "../utils/constants";
import { KeyPairFile, loadKeypair } from "../utils/helpers";
import { findAssetManagerAddress, findListingDataAddress } from "../utils/pda";
import { ListProgram } from "../utils/programs";

describe("LIST PROGRAM", () => {
	// get the signer keypair
	let signer = loadKeypair(KeyPairFile.main);

	// instantiate LIST Program, using default provider
	const listProgram = new ListProgram();
	const program = listProgram.getProgram();

	// --------------------------------------------------------------------------ADMIN IXs

	// it("Initializes escrow account!", async () => {
	// 	const txHash = await program.methods
	// 		.initEscrowAccount()
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			systemProgram: SystemProgram.programId,
	// 			assetManager: findAssetManagerAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Lists an MPL core asset!", async () => {
	// 	const txHash = await program.methods
	// 		.listAsset({
	// 			amount: new BN(1_0000_000),
	// 			paymentOption: { native: {} },
	// 		})
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			asset: new PublicKey(
	// 				"9yjYqYrEmFYkJeyby8Xaw5bvTVthqqUmSt8XcRuUH3uc"
	// 			),
	// 			listingData: findListingDataAddress(),
	// 			assetManager: findAssetManagerAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("UnLists an MPL core asset!", async () => {
	// 	const txHash = await program.methods
	// 		.unlistAsset()
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			asset: new PublicKey(
	// 				"9yjYqYrEmFYkJeyby8Xaw5bvTVthqqUmSt8XcRuUH3uc"
	// 			),
	// 			listingData: findListingDataAddress(),
	// 			assetManager: findAssetManagerAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });
});
