import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import * as anchor from '@coral-xyz/anchor';
import {Program} from '@coral-xyz/anchor';
import {PublicKey} from '@solana/web3.js';
import {Voting} from '../target/types/voting';
import exp from "constants";

const IDL = require("../target/idl/voting.json");
const votingAddress = new PublicKey("AsjZ3kWAUSQRNt2pZVeJkywhZ6gpLpHZmJjduPmKZDZZ");

describe('voting', () => {

  let context;
  let provider;
  let votingProgram: anchor.Program<Voting>;
  
  beforeAll( async () => {
    context = await startAnchor("", [{name: "voting", programId: votingAddress}], []);
    provider = new BankrunProvider(context);
    votingProgram = new Program<Voting>(
      IDL,
		  provider,
	  );

  });

  it("Initialize Poll", async () => {
    await votingProgram.methods.initializePoll(
      new anchor.BN(1),
      "What color is the dress?",
      new anchor.BN(0),
      new anchor.BN(1738801520)
    ).rpc();

    const [pollAddress] = PublicKey.findProgramAddressSync([new anchor.BN(1).toArrayLike(Buffer,'le',8)], votingAddress);

    const poll = await votingProgram.account.pollAcount.fetch(pollAddress);

    console.log(poll);

    expect(poll.pollId.toNumber()).toEqual(1);
    expect(poll.pollDescription).toEqual("What color is the dress?");
    expect(poll.pollStart.toNumber()).toBeLessThan(poll.pollEnd.toNumber());

  });

  it("Initialize candidate", async () => {

    await votingProgram.methods.initializeCandidate(
      new anchor.BN(1),
      "blue"
    ).rpc();

    const [blueAddress] = PublicKey.findProgramAddressSync([new anchor.BN(1).toArrayLike(Buffer,'le',8), Buffer.from("blue")], votingAddress);

    const blueCandidate = await votingProgram.account.candidate.fetch(blueAddress);

    expect(blueCandidate.candidateVotes.toNumber()).toEqual(0);

    await votingProgram.methods.initializeCandidate(
      new anchor.BN(1),
      "white"
    ).rpc();

    const [whiteAddress] = PublicKey.findProgramAddressSync( [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("white")], votingAddress);

    const whiteCandidate = await votingProgram.account.candidate.fetch(whiteAddress);

    expect(whiteCandidate.candidateVotes.toNumber()).toEqual(0);    
  });

  it("vote", async () => {
    await votingProgram.methods.vote(
      new anchor.BN(1),
      "white"
    ).rpc();

    const [whiteAddress] = PublicKey.findProgramAddressSync( [new anchor.BN(1).toArrayLike(Buffer, 'le', 8), Buffer.from("white")], votingAddress);

    const whiteCandidate = await votingProgram.account.candidate.fetch(whiteAddress);

    expect(whiteCandidate.candidateVotes.toNumber()).toEqual(1);
  })
  
})

