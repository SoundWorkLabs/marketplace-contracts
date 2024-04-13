import { AnchorProvider, setProvider } from "@coral-xyz/anchor";

import { Keypair, SystemProgram } from "@solana/web3.js";
import { CORE_PROGRAM_ID } from "./utils/constants";
import { KeyPairFile, loadKeypair } from "./utils/helpers";
import { CreateProgram } from "./utils/programs";

describe("CREATE PROGRAM", () => {
	// set default provider
	setProvider(AnchorProvider.env());

	// get the signer keypair
	let signer = loadKeypair(KeyPairFile.main);

	// instantiate CREATE Program, using default provider
	const createProgram = new CreateProgram();
	const program = createProgram.getProgram();

	it("Is Mints a Core Asset!", async () => {
		const asset = Keypair.generate();

		const metadata = {
			name: "Kobeni Supremacy",
			uri: "https://raw.githubusercontent.com/687c/solana-nft-native-client/main/metadata.json",
		};

		const txHash = await program.methods
			.create(metadata.name, metadata.uri)
			.accounts({
				payer: signer.publicKey,
				systemProgram: SystemProgram.programId,
				asset: asset.publicKey,
				coreProgram: CORE_PROGRAM_ID,
			})
			.signers([asset])
			.rpc();

		console.log(
			`mint Address: https://explorer.solana.com/address/${asset.publicKey}?cluster=devnet\n`
		);
		console.log(
			`mint tx: https://explorer.solana.com/tx/${txHash}?cluster=devnet`
		);
	});
});
