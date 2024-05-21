import { AnchorProvider, setProvider } from "@coral-xyz/anchor";

import { ASSOCIATED_PROGRAM_ID } from "@coral-xyz/anchor/dist/cjs/utils/token";
import {
	getAssociatedTokenAddressSync,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
	Keypair,
	LAMPORTS_PER_SOL,
	PublicKey,
	SystemProgram,
} from "@solana/web3.js";
import { BN } from "bn.js";
import { CORE_PROGRAM_ID, PAYMENT_MINT } from "../utils/constants";
import { KeyPairFile, loadKeypair } from "../utils/helpers";
import {
	findAssetManagerAddress,
	findListingDataAddress,
	findMarketplaceConfigAddress,
	findWalletAddress,
} from "../utils/pda";
import { ListProgram } from "../utils/programs";

describe("LIST PROGRAM", () => {
	// get the signer keypair
	let signer = loadKeypair(KeyPairFile.main);

	// instantiate LIST Program, using default provider
	const listProgram = new ListProgram();
	const program = listProgram.getProgram();

	// ! get rid of me below
	let asset = new PublicKey("4tJLLXxZEj74n1m2CfQKMBjAZAg8t7bMpXnze3FPpxns");

	// --------------------------------------------------------------------------ADMIN IXs

	// it("Initializes asset manager escrow account!", async () => {
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

	// it("Initializes Marketplace config account!", async () => {
	// 	const txHash = await program.methods
	// 		.initMarketplaceConfigAccount({
	// 			takerFeeBps: 1,
	// 			treasuryAddress: signer.publicKey, // todo: update me
	// 		})
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			marketplaceConfig: findMarketplaceConfigAddress(),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// --------------------------------------------------------------------------USER IXs

	// it("Initializes user wallet escrow!", async () => {
	// 	const txHash = await program.methods
	// 		.initUserEscrowWallet()
	// 		.accounts({
	// 			authority: signer.publicKey,
	// 			wallet: findWalletAddress(signer.publicKey),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Deposits SOL to user escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.depositSol({ amount: new BN(1 * LAMPORTS_PER_SOL) })
	// 		.accounts({
	// 			authority: signer.publicKey,
	// 			wallet: findWalletAddress(signer.publicKey),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Withdraws SOL to user escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.withdrawSol({ amount: new BN(0.5 * LAMPORTS_PER_SOL) })
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			authority: signer.publicKey,
	// 			wallet: findWalletAddress(signer.publicKey),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Deposits Tokens to user escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.depositToken({ amount: new BN(1_000_000) }) // with 6 decimals
	// 		.accounts({
	// 			authority: signer.publicKey,
	// 			wallet: findWalletAddress(signer.publicKey),
	// 			mint: PAYMENT_MINT,
	// 			authorityTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				signer.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(signer.publicKey),
	// 				true
	// 			),
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Withdraws Tokens from user escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.withdrawToken({ amount: new BN(1) })
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			authority: signer.publicKey,
	// 			wallet: findWalletAddress(signer.publicKey),
	// 			mint: PAYMENT_MINT,
	// 			authorityTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				signer.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(signer.publicKey),
	// 				true
	// 			),
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// --------------------------------------------------------------------------LIST IXs

	// it("Lists an MPL core asset!", async () => {
	// 	const txHash = await program.methods
	// 		.listAsset({
	// 			amount: new BN(1_0000_000),
	// 			paymentOption: { native: {} },
	// 		})
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			asset,
	// 			listingData: findListingDataAddress(asset),
	// 			assetManager: findAssetManagerAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("UnLists an MPL core asset!", async () => {
	// 	const txHash = await program.methods
	// 		.unlistAsset()
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			asset,
	// 			listingData: findListingDataAddress(asset),
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
