import { AnchorProvider, setProvider } from "@coral-xyz/anchor";

import {
	ASSOCIATED_TOKEN_PROGRAM_ID,
	getAssociatedTokenAddressSync,
	TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
	Keypair,
	LAMPORTS_PER_SOL,
	sendAndConfirmTransaction,
	SystemProgram,
	Transaction,
} from "@solana/web3.js";
import { BN } from "bn.js";
import {
	asset,
	CORE_PROGRAM_ID,
	PAYMENT_MINT,
	SOUNDWORK_LIST_ID,
} from "../utils/constants";
import { KeyPairFile, loadKeypair } from "../utils/helpers";
import {
	findAssetManagerAddress,
	findBidDataAddress,
	findListingDataAddress,
	findMarketplaceConfigAddress,
	findWalletAddress,
} from "../utils/pda";
import { BidProgram, ListProgram } from "../utils/programs";

describe("BID PROGRAM", () => {
	// get the signer keypairs
	let signer = loadKeypair(KeyPairFile.main); // used as treasury

	let seller = loadKeypair(KeyPairFile.main);
	let bidder = loadKeypair(KeyPairFile.secondary);

	// instantiate List Program,
	const listProgram = new ListProgram().getProgram();
	// const program = listProgram.getProgram();

	// instantiate BID Program,
	const bidProgram = new BidProgram();
	const program = bidProgram.getProgram();

	// --------------------------------------------------------------------------BID IXs

	// 	// wants to be paid using tokens
	// 	it("Makes a bid on a listed asset!", async () => {
	// 		let expiryTs = new BN(new Date().getTime());

	// 		/* 	const initWalletIX = await listProgram.methods
	// 			.initUserEscrowWallet()
	// 			.accounts({
	// 				authority: bidder.publicKey,
	// 				wallet: findWalletAddress(bidder.publicKey),
	// 				systemProgram: SystemProgram.programId,
	// 			})
	// 			.instruction();
	//  */
	// 		const bidIX = await program.methods
	// 			.makeBid({
	// 				amount: new BN(1 * LAMPORTS_PER_SOL),
	// 				expiryTs,
	// 			})
	// 			.accounts({
	// 				bidder: bidder.publicKey,
	// 				asset,
	// 				bidData: findBidDataAddress(asset),
	// 				bidderEscrowWallet: findWalletAddress(bidder.publicKey),
	// 				listingData: findListingDataAddress(asset),
	// 				paymentMint: PAYMENT_MINT,
	// 				bidderTokenAccount: getAssociatedTokenAddressSync(
	// 					PAYMENT_MINT,
	// 					bidder.publicKey
	// 				),
	// 				// in sdk check that this is initialized and if not, call initialize wallet
	// 				// initializeBidderWalletAndBid()
	// 				walletTokenAccount: getAssociatedTokenAddressSync(
	// 					PAYMENT_MINT,
	// 					findWalletAddress(bidder.publicKey),
	// 					true
	// 				),
	// 				soundworkList: SOUNDWORK_LIST_ID,
	// 				associatedTokenProgram: ASSOCIATED_PROGRAM_ID,
	// 				systemProgram: SystemProgram.programId,
	// 			})
	// 			.instruction();

	// const tx = new Transaction() /* .add(initWalletIX) */
	// 	.add(bidIX);

	// let txHash = await sendAndConfirmTransaction(
	// 	program.provider.connection,
	// 	tx,
	// 	[bidder],
	// 	{
	// 		skipPreflight: true,
	// 	}
	// );

	// 		console.log(
	// 			`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 		);
	// 	});

	// it("Edits a bid for a listed asset!", async () => {
	// 	let ix = await program.methods
	// 		.editBid({ amount: new BN(1000), expiryTs: null })
	// 		.accounts({
	// 			bidder: bidder.publicKey,
	// 			asset,
	// 			bidData: findBidDataAddress(asset),
	// 			bidderEscrowWallet: findWalletAddress(bidder.publicKey),
	// 			listingData: findListingDataAddress(asset),
	// 			paymentMint: PAYMENT_MINT,
	// 			bidderTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				bidder.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(bidder.publicKey),
	// 				true
	// 			),
	// 			soundworkList: SOUNDWORK_LIST_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.instruction();

	// 	const tx = new Transaction().add(ix);

	// 	let txHash = await sendAndConfirmTransaction(
	// 		program.provider.connection,
	// 		tx,
	// 		[bidder],
	// 		{
	// 			skipPreflight: true,
	// 		}
	// 	);

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Revoked bid on a listed asset!", async () => {
	// 	let ix = await program.methods
	// 		.revokeBid()
	// 		.accounts({
	// 			bidder: bidder.publicKey,
	// 			asset,
	// 			bidData: findBidDataAddress(asset),
	// 			bidderEscrowWallet: findWalletAddress(bidder.publicKey),
	// 			listingData: findListingDataAddress(asset),
	// 			paymentMint: PAYMENT_MINT,
	// 			bidderTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				bidder.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(bidder.publicKey),
	// 				true
	// 			),
	// 			soundworkList: SOUNDWORK_LIST_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.instruction();

	// 	const tx = new Transaction().add(ix);

	// 	let txHash = await sendAndConfirmTransaction(
	// 		program.provider.connection,
	// 		tx,
	// 		[bidder],
	// 		{
	// 			skipPreflight: true,
	// 		}
	// 	);

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });

	// it("Accepts a bid for a listed asset!", async () => {
	// 	let txHash = await program.methods
	// 		.acceptBid()
	// 		.accounts({
	// 			seller: seller.publicKey,
	// bidder: bidder.publicKey,
	// asset,
	// 			bidData: findBidDataAddress(asset),
	// 			bidderEscrowWallet: findWalletAddress(bidder.publicKey),
	// 			listingData: findListingDataAddress(asset),
	// 			paymentMint: PAYMENT_MINT,
	// bidderTokenAccount: getAssociatedTokenAddressSync(
	// 	PAYMENT_MINT,
	// 	bidder.publicKey
	// ),
	// 			sellerTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				seller.publicKey
	// 			),
	// walletTokenAccount: getAssociatedTokenAddressSync(
	// 	PAYMENT_MINT,
	// 	findWalletAddress(bidder.publicKey),
	// 	true
	// ),
	// 			treasuryTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				signer.publicKey
	// 			),
	// 			treasury: signer.publicKey,
	// 			assetManager: findAssetManagerAddress(),
	// 			marketplaceConfig: findMarketplaceConfigAddress(),
	// 			soundworkList: SOUNDWORK_LIST_ID,
	// 			coreProgram: CORE_PROGRAM_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc({ skipPreflight: true });

	// console.log(
	// 	`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// );
	// });

	// it("Seller rejects bid on listed asset!", async () => {
	// 	let ix = await program.methods
	// 		.rejectBid()
	// 		.accounts({
	// 			seller: seller.publicKey,
	// 			bidder: bidder.publicKey,
	// 			asset,
	// 			bidData: findBidDataAddress(asset),
	// 			bidderEscrowWallet: findWalletAddress(bidder.publicKey),
	// 			listingData: findListingDataAddress(asset),
	// 			paymentMint: PAYMENT_MINT,
	// 			bidderTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				bidder.publicKey
	// 			),
	// 			walletTokenAccount: getAssociatedTokenAddressSync(
	// 				PAYMENT_MINT,
	// 				findWalletAddress(bidder.publicKey),
	// 				true
	// 			),
	// 			soundworkList: SOUNDWORK_LIST_ID,
	// 			tokenProgram: TOKEN_PROGRAM_ID,
	// 			associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
	// 			systemProgram: SystemProgram.programId,
	// 		})
	// 		.rpc();

	// 	console.log(
	// 		`tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet\n`
	// 	);
	// });
});
