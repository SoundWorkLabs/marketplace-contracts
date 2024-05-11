import { AnchorProvider, setProvider } from "@coral-xyz/anchor";

import { Keypair, SystemProgram } from "@solana/web3.js";
import { CORE_PROGRAM_ID } from "../utils/constants";
import { KeyPairFile, loadKeypair } from "../utils/helpers";
import { findAssetManagerAddress } from "../utils/pda";
import { ListProgram } from "../utils/programs";

describe("BID PROGRAM", () => {
	// get the signer keypair
	let signer = loadKeypair(KeyPairFile.main);

	// instantiate BID Program,
	const bidProgram = new ListProgram();
	const program = bidProgram.getProgram();

	// --------------------------------------------------------------------------ADMIN IXs

	it("Initializes bid manager!", async () => {});
});
