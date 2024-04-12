import { Idl } from "@coral-xyz/anchor";
import { Keypair } from "@solana/web3.js";
import { readFileSync } from "fs";

export function loadProgramIdl(program: SupportedPrograms): Idl {
	switch (program) {
		case "bid": {
			return JSON.parse(
				readFileSync("../../target/idl/soundwork_bid.json", "utf8")
			);
		}
		case "create": {
			return JSON.parse(
				readFileSync("../../target/idl/soundwork_create.json", "utf8")
			);
		}
		case "list": {
			return JSON.parse(
				readFileSync("../../target/idl/soundwork_list.json", "utf8")
			);
		}
		default: {
			console.log("error: unknown program");
			break;
		}
	}
}

type SupportedPrograms = "bid" | "create" | "list";

export async function loadKeypair(filePath: string): Promise<Keypair> {
	const data = readFileSync(filePath, "utf-8");
	return Keypair.fromSecretKey(Buffer.from(JSON.parse(data)));
}
