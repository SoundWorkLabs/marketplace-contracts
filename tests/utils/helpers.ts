import { Idl } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { readFileSync } from "fs";
import { homedir } from "os";

export function loadProgramIdl(program: SupportedPrograms): Idl {
	switch (program) {
		case "bid": {
			return JSON.parse(
				readFileSync(
					process.cwd() + "/target/idl/soundwork_bid.json",
					"utf8"
				)
			);
		}
		case "create": {
			return JSON.parse(
				readFileSync(
					process.cwd() + "/target/idl/soundwork_create.json",
					"utf8"
				)
			);
		}
		case "list": {
			return JSON.parse(
				readFileSync(
					process.cwd() + "/target/idl/soundwork_list.json",
					"utf8"
				)
			);
		}
		default: {
			console.log("error: unknown program");
			break;
		}
	}
}

type SupportedPrograms = "bid" | "create" | "list";

export function loadKeypair(file: KeyPairFile): Keypair {
	const data = readFileSync(homedir() + `/.config/solana/${file}`, "utf-8");

	return Keypair.fromSecretKey(Buffer.from(JSON.parse(data)));
}

// ! change this accordingly

export enum KeyPairFile {
	main = "id.json",
	secondary = "id-new.json",
}
