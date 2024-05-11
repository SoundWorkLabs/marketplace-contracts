// our program instances

import { AnchorProvider, Program, Provider } from "@coral-xyz/anchor";

import { SoundworkBid } from "../../target/types/soundwork_bid";
import { SoundworkList } from "../../target/types/soundwork_list";
import { SoundworkCreate } from "../../target/types/soundwork_create";
import {
	defaultProvider,
	SOUNDWORK_BID_ID,
	SOUNDWORK_CREATE_ID,
	SOUNDWORK_LIST_ID,
} from "./constants";
import { loadProgramIdl } from "./helpers";

export class BidProgram {
	constructor(readonly provider: Provider = defaultProvider) {}

	getProgram(): Program<SoundworkBid> {
		const idl = loadProgramIdl("bid");
		const programId = SOUNDWORK_BID_ID;

		return new Program(
			idl,
			programId,
			this.provider
		) as Program<SoundworkBid>;
	}
}

export class CreateProgram {
	constructor(readonly provider: Provider = defaultProvider) {}

	getProgram(): Program<SoundworkCreate> {
		const idl = loadProgramIdl("create");
		const programId = SOUNDWORK_CREATE_ID;

		return new Program(
			idl,
			programId,
			this.provider
		) as Program<SoundworkCreate>;
	}
}

export class ListProgram {
	constructor(readonly provider: Provider = defaultProvider) {}

	getProgram(): Program<SoundworkList> {
		const idl = loadProgramIdl("list");
		const programId = SOUNDWORK_LIST_ID;

		return new Program(
			idl,
			programId,
			this.provider
		) as Program<SoundworkList>;
	}
}
