// our program instances

import { AnchorProvider, Program, Provider } from "@coral-xyz/anchor";

import type { SoundworkBid } from "../../target/types/soundwork_bid";
import type { SoundworkCreate } from "../../target/types/soundwork_create";
import type { SoundworkList } from "../../target/types/soundwork_list";

// @ts-ignore
import * as soundworkBidIDL from "../../target/idl/soundwork_bid.json";
// @ts-ignore
import * as soundworkCreateIDL from "../../target/idl/soundwork_create.json";
// @ts-ignore
import * as soundworkListIDL from "../../target/idl/soundwork_list.json";

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

		return new Program<SoundworkBid>(
			soundworkListIDL as unknown as soundworkBidIDL,
			this.provider
		);
	}
}

export class CreateProgram {
	constructor(readonly provider: Provider = defaultProvider) {}

	getProgram(): Program<SoundworkCreate> {
		return new Program<SoundworkCreate>(
			soundworkCreateIDL as unknown as SoundworkCreate,
			this.provider
		);
	}
}

export class ListProgram {
	constructor(readonly provider: Provider = defaultProvider) {}

	getProgram(): Program<SoundworkList> {
		return new Program<SoundworkList>(
			soundworkListIDL as unknown as SoundworkList,
			this.provider
		);
	}
}
