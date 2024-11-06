import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SplitBill } from "../target/types/split_bill";
import { PublicKey, Connection, LAMPORTS_PER_SOL, Keypair } from '@solana/web3.js';
import { 
  TOKEN_PROGRAM_ID, 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  createMint, 
  getOrCreateAssociatedTokenAccount,
  mintTo,
  getAccount
} from "@solana/spl-token";
import { expect } from "chai";

describe("split-bill", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SplitBill as Program<SplitBill>;
  
  let payer: Keypair;
  let contributor1: Keypair;
  let contributor2: Keypair;
  let receiver: Keypair;
  let splitTokenAccount: PublicKey;
  let mint: PublicKey;
  let contributorTokenAccount1: PublicKey;
  let contributorTokenAccount2: PublicKey;
  let receiverTokenAccount: PublicKey;
  let splitBillAddress: PublicKey;

  const BILL_NAME = "test_bill";
  const TOTAL_AMOUNT = 1000;

  beforeEach(async () => {
    payer = Keypair.generate();
    contributor1 = Keypair.generate();
    contributor2 = Keypair.generate();
    receiver = Keypair.generate();

    await provider.connection.requestAirdrop(payer.publicKey, 10 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(contributor1.publicKey, 5 * LAMPORTS_PER_SOL);
    await provider.connection.requestAirdrop(contributor2.publicKey, 5 * LAMPORTS_PER_SOL);
    
    await new Promise(resolve => setTimeout(resolve, 1000));

    try {
      mint = await createMint(
        provider.connection,
        payer,
        payer.publicKey,
        null,
        6,
        undefined,
        undefined,
        TOKEN_PROGRAM_ID
      );

      console.log("Mint created:", mint.toBase58());

      const splitTokenAccountInfo = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        payer.publicKey
      );
      splitTokenAccount = splitTokenAccountInfo.address;

      const contributorTokenAccountInfo1 = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        contributor1.publicKey
      );
      contributorTokenAccount1 = contributorTokenAccountInfo1.address;

      const contributorTokenAccountInfo2 = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        contributor2.publicKey
      );
      contributorTokenAccount2 = contributorTokenAccountInfo2.address;

      const receiverTokenAccountInfo = await getOrCreateAssociatedTokenAccount(
        provider.connection,
        payer,
        mint,
        receiver.publicKey
      );
      receiverTokenAccount = receiverTokenAccountInfo.address;

      await mintTo(
        provider.connection,
        payer,
        mint,
        contributorTokenAccount1,
        payer.publicKey,
        500
      );

      await mintTo(
        provider.connection,
        payer,
        mint,
        contributorTokenAccount2,
        payer.publicKey,
        500
      );

      [splitBillAddress] = PublicKey.findProgramAddressSync(
        [payer.publicKey.toBuffer(), Buffer.from(BILL_NAME)],
        program.programId
      );

    } catch (error) {
      console.error("Setup error:", error);
      throw error;
    }
  });

  it("Initialize a split bill", async () => {
    try {
      const tx = await program.methods
        .initializeSplit(
          payer.publicKey, 
          BILL_NAME, 
          new anchor.BN(TOTAL_AMOUNT)
        )
        .accountsPartial({
          initializer: payer.publicKey,
          bill: splitBillAddress,
          receiver: receiver.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer])
        .rpc();

      const splitBill = await program.account.splitBill.fetch(splitBillAddress);

      expect(splitBill.billName).to.equal(BILL_NAME);
      expect(splitBill.totalAmount.toNumber()).to.equal(TOTAL_AMOUNT);
      expect(splitBill.authority.toBase58()).to.equal(payer.publicKey.toBase58());
      expect(splitBill.contributors.length).to.equal(0);
      expect(splitBill.isSettled).to.equal(false);
    } catch (error) {
      console.error("Initialize split bill error:", error);
      throw error;
    }
  });

  it("Add first contributor", async () => {
    // First initialize the bill
    await program.methods
      .initializeSplit(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(TOTAL_AMOUNT)
      )
      .accountsPartial({
        initializer: payer.publicKey,
        bill: splitBillAddress,
        receiver: receiver.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    try {
      const tx = await program.methods
        .contribute(
          payer.publicKey, 
          BILL_NAME, 
          new anchor.BN(500)
        )
        .accountsPartial({
          bill: splitBillAddress,
          token: mint,
          contributor: contributor1.publicKey,
          contributorTokenAccount: contributorTokenAccount1,
          splitTokenAccount: splitTokenAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([contributor1])
        .rpc();

      const splitBill = await program.account.splitBill.fetch(splitBillAddress);
      const splitTokenAccountInfo = await getAccount(provider.connection, splitTokenAccount);

      expect(splitBill.contributors.length).to.equal(1);
      expect(splitBill.contributors[0].address.toBase58()).to.equal(contributor1.publicKey.toBase58());
      expect(splitBill.contributors[0].amount.toNumber()).to.equal(500);
      expect(splitBill.isSettled).to.equal(false);
      expect(Number(splitTokenAccountInfo.amount)).to.equal(500);
    } catch (error) {
      console.error("Add first contributor error:", error);
      throw error;
    }
  });

  it("Complete bill settlement and withdrawal flow", async () => {
    // Initialize the bill
    await program.methods
      .initializeSplit(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(TOTAL_AMOUNT)
      )
      .accountsPartial({
        initializer: payer.publicKey,
        bill: splitBillAddress,
        receiver: receiver.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    // Add first contributor
    await program.methods
      .contribute(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(500)
      )
      .accountsPartial({
        bill: splitBillAddress,
        token: mint,
        contributor: contributor1.publicKey,
        contributorTokenAccount: contributorTokenAccount1,
        splitTokenAccount: splitTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([contributor1])
      .rpc();

    // Add second contributor
    await program.methods
      .contribute(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(500)
      )
      .accountsPartial({
        bill: splitBillAddress,
        token: mint,
        contributor: contributor2.publicKey,
        contributorTokenAccount: contributorTokenAccount2,
        splitTokenAccount: splitTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([contributor2])
      .rpc();

    // Verify bill is settled
    let splitBill = await program.account.splitBill.fetch(splitBillAddress);
    expect(splitBill.isSettled).to.equal(true);

    // Withdraw funds
    await program.methods
      .withdraw()
      .accountsPartial({
        bill: splitBillAddress,
        authority: payer.publicKey,
        splitTokenAccount: splitTokenAccount,
        receiverTokenAccount: receiverTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([payer])
      .rpc();

    // Verify final balances
    const receiverTokenAccountInfo = await getAccount(provider.connection, receiverTokenAccount);
    const splitTokenAccountInfo = await getAccount(provider.connection, splitTokenAccount);

    expect(Number(receiverTokenAccountInfo.amount)).to.equal(1000);
    expect(Number(splitTokenAccountInfo.amount)).to.equal(0);
  });

  it("Should fail to contribute after bill is settled", async () => {
    // Initialize the bill
    await program.methods
      .initializeSplit(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(TOTAL_AMOUNT)
      )
      .accountsPartial({
        initializer: payer.publicKey,
        bill: splitBillAddress,
        receiver: receiver.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([payer])
      .rpc();

    // Add both contributors to settle the bill
    await program.methods
      .contribute(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(500)
      )
      .accountsPartial({
        bill: splitBillAddress,
        token: mint,
        contributor: contributor1.publicKey,
        contributorTokenAccount: contributorTokenAccount1,
        splitTokenAccount: splitTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([contributor1])
      .rpc();

    await program.methods
      .contribute(
        payer.publicKey, 
        BILL_NAME, 
        new anchor.BN(500)
      )
      .accountsPartial({
        bill: splitBillAddress,
        token: mint,
        contributor: contributor2.publicKey,
        contributorTokenAccount: contributorTokenAccount2,
        splitTokenAccount: splitTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
      .signers([contributor2])
      .rpc();

    // Try to contribute after settlement
    try {
      await program.methods
        .contribute(
          payer.publicKey, 
          BILL_NAME, 
          new anchor.BN(100)
        )
        .accountsPartial({
          bill: splitBillAddress,
          token: mint,
          contributor: contributor1.publicKey,
          contributorTokenAccount: contributorTokenAccount1,
          splitTokenAccount: splitTokenAccount,
          systemProgram: anchor.web3.SystemProgram.programId,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
        .signers([contributor1])
        .rpc();
      
      expect.fail("Should have thrown an error");
    } catch (error) {
      expect(error.message).to.include("Bill is already settled");
    }
  });
});