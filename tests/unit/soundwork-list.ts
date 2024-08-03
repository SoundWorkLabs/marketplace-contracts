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
import { asset, CORE_PROGRAM_ID, PAYMENT_MINT } from "../utils/constants";
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

	let seller = loadKeypair(KeyPairFile.main);
	let buyer = loadKeypair(KeyPairFile.secondary);

	// instantiate LIST Program, using default provider
	const listProgram = new ListProgram();
	const program = listProgram.getProgram();

	// ! get rid of me below
	// let asset = new PublicKey("H4gutS7fRDgb4c4sDhULvQ23PaN81d5qgQpkaapC7N8t");

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
	// 		.rpc({skipPreflight: true});

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

	// it("Initializes buyer wallet escrow account!", async () => {
	// 	const txHash = await program.methods
	// 		.initUserEscrowWallet()
	// 		.accounts({
	// 			authority: buyer.publicKey,
	// 			wallet: findWalletAddress(buyer.publicKey),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Deposits SOL to buyer escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.depositSol({ amount: new BN(1 * LAMPORTS_PER_SOL) })
	// 		.accounts({
	// 			authority: buyer.publicKey,
	// 			wallet: findWalletAddress(buyer.publicKey),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Withdraws SOL from buyer escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.withdrawSol({ amount: new BN(0.01 * LAMPORTS_PER_SOL) })
	// 		.accounts({
	// 			payer: buyer.publicKey,
	// 			authority: buyer.publicKey,
	// 			wallet: findWalletAddress(buyer.publicKey),
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Deposits Tokens to buyer escrow wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.depositToken({ amount: new BN(1_000_000) }) // with 6 decimals, this is 1 USDC dev coin
	// 		.accounts({
	// 			authority: buyer.publicKey,
	// 			wallet: findWalletAddress(buyer.publicKey),
	// 			mint: PAYMENT_MINT,
	// 			authorityTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				buyer.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(buyer.publicKey),
	// 				true
	// 			),
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Withdraws Tokens from buyer escrow wallet", async () => {
	// 	const txHash = await program.methods
	// 		.withdrawToken({ amount: new BN(1) })
	// 		.accounts({
	// 			payer: buyer.publicKey,
	// 			authority: buyer.publicKey,
	// 			wallet: findWalletAddress(buyer.publicKey),
	// 			mint: PAYMENT_MINT,
	// 			authorityTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				buyer.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(buyer.publicKey),
	// 				true
	// 			),
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// --------------------------------------------------------------------------LIST IXs

	// it("Lists an MPL core asset with native payment option!", async () => {
	// 	const txHash = await program.methods
	// 		.listAsset({
	// 			amount: new BN(1_0000_000),
	// 			paymentOption: { native: {} },
	// 		})
	// 		.accounts({
	// 			payer: signer.publicKey,
	// 			asset,
	// 			collection: null,
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

	// it("Lists an MPL core asset and uses tokens as payment option!", async () => {
	// 	const txHash = await program.methods
	// 		.listAsset({
	// 			amount: new BN(1_000_000), // 1 USDC dev
	// 			paymentOption: { token: { mint: PAYMENT_MINT } },
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
	// 			collection: null,
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

	// it("Buys a listed NFT using Native SOL!", async () => {
	// 	const txHash = await program.methods
	// 		.buyAsset(null) // ! fails as expected
	// 		.accounts({
	// 			payer: buyer.publicKey,
	// 			buyer: buyer.publicKey,
	// 			seller: seller.publicKey,
	// 			walletAsBuyer: null,
	// 			asset,
	// 			paymentMint: null,
	// 			walletTokenAccount: null,
	// 			buyerTokenAccount: null,
	// 			sellerTokenAccount: null,
	// 			treasuryTokenAccount: null, // ! update to correct address
	// 			treasury: signer.publicKey, // ! update to correct address
	// 			listingData: findListingDataAddress(asset),
	// 			assetManager: findAssetManagerAddress(),
	// 			marketplaceConfig: findMarketplaceConfigAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	//	associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Buys a listed NFT using Tokens using Escrow Wallet!", async () => {
	// 	const txHash = await program.methods
	// 		.buyAsset(null) // ! fails as expected
	// 		// .buyAsset({ bidAmount: new BN(1_000_000) }) // ! works, but this is for seller when accepting bids
	// 		.accounts({
	// 			payer: buyer.publicKey,
	// 			buyer: buyer.publicKey,
	// 			seller: seller.publicKey,
	// 			walletAsBuyer: findWalletAddress(buyer.publicKey),
	// 			// walletAsBuyer: null,

	// 			asset,
	// 			paymentMint: PAYMENT_MINT,
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(buyer.publicKey),
	// 				true
	// 			),
	// 			buyerTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				buyer.publicKey
	// 			),
	// 			sellerTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				seller.publicKey
	// 			),
	// 			treasuryTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				signer.publicKey
	// 			), // ! update to correct address
	// 			treasury: signer.publicKey, // ! update to correct address
	// 			listingData: findListingDataAddress(asset),
	// 			assetManager: findAssetManagerAddress(),
	// 			marketplaceConfig: findMarketplaceConfigAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 	associatedTokenProgram: ASSOCIATED_PROGRAM_ID,

	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Buys a listed NFT using Tokens!", async () => {
	// 	const txHash = await program.methods
	// 		.buyAsset(null) // ! fails as expected
	// 		.accounts({
	// 			payer: buyer.publicKey,
	// 			buyer: buyer.publicKey,
	// 			seller: seller.publicKey,
	// 			walletAsBuyer: null,

	// 			asset,
	// 			paymentMint: PAYMENT_MINT,
	// 			walletTokenAccount: null,
	// 			buyerTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				buyer.publicKey
	// 			),
	// 			sellerTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				seller.publicKey
	// 			),
	// 			treasuryTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				signer.publicKey
	// 			), // ! update to correct address
	// 			treasury: signer.publicKey, // ! update to correct address
	// 			listingData: findListingDataAddress(asset),
	// 			assetManager: findAssetManagerAddress(),
	// 			marketplaceConfig: findMarketplaceConfigAddress(),
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.signers([buyer])
	// 		.rpc({ skipPreflight: true });

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });
});
