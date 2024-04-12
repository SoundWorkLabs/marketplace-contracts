// our program instances

import { AnchorProvider, Program, Provider } from "@coral-xyz/anchor";

import { SoundworkBid } from "../../target/types/soundwork_bid";
import { SoundworkList } from "../../target/types/soundwork_list";

import { SoundworkCreate } from "../../target/types/soundwork_create";
import {
	defaultProvider,
	SOUNDWORK_BID,
	SOUNDWORK_CREATE,
	SOUNDWORK_LIST,
} from "./constants";
import { loadProgramIdl } from "./helpers";

export class BidProgram {
	constructor(readonly provider: Provider = defaultProvider) {
		new Program(
			loadProgramIdl("bid"),
			SOUNDWORK_BID,
			provider
		) as Program<SoundworkBid>;
	}
}

export class CreateProgram {
	constructor(readonly provider: Provider = defaultProvider) {
		new Program(
			loadProgramIdl("create"),
			SOUNDWORK_CREATE,
			provider
		) as Program<SoundworkCreate>;
	}
}

export class ListProgram {
	constructor(readonly provider: Provider = defaultProvider) {
		new Program(
			loadProgramIdl("list"),
			SOUNDWORK_LIST,
			provider
		) as Program<SoundworkList>;
	}
}
