import { AnchorProvider } from "@coral-xyz/anchor";

import { CreateProgram } from "./utils/programs";

describe("CREATE PROGRAM", async () => {
	// set default provider
	anchor.setProvider(anchor.AnchorProvider.env());

	// instantiate CREATE Program, using default provider
	const program = new CreateProgram();
});
