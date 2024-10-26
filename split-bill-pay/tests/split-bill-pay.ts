import { startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import * as anchor from '@coral-xyz/anchor';
import {Program} from '@coral-xyz/anchor';
import {PublicKey} from '@solana/web3.js';
import {SplitBillPay} from '../target/types/split_bill_pay';
import { expect } from "chai";

const IDL = require("../target/idl/split_bill_pay.json");
const SPLITBILLPAY_ADDRESS = new PublicKey("7wQx44y6ggAPVLBgPm7RuZCpTbYsqerJ4v7Yw2x3MyuU");

describe("split-bill-pay", () => {

  let context;
  let provider;
  let program: Program<SplitBillPay>;
  let splitBillAccount = anchor.web3.Keypair.generate();
  let receiverAccount = anchor.web3.Keypair.generate();
  let contributor1 = anchor.web3.Keypair.generate();
  let contributor2 = anchor.web3.Keypair.generate();


  before(async () => {
    context = await startAnchor("",[{name:"split_bill_pay",programId:SPLITBILLPAY_ADDRESS}],[]);
    provider = new BankrunProvider(context);

    program = new Program<SplitBillPay>(IDL, provider);
  });

  it("Initialize split", async () => {
    await program.methods.initializeSplit(new anchor.BN(1000000000)).accountsPartial({
      receiver: receiverAccount.publicKey,
    }).rpc();

    const splitBill = await program.account.splitBill.fetch(SPLITBILLPAY_ADDRESS);

    expect(splitBill.authority).to.equal(provider.wallet.publicKey);
    expect(splitBill.totalAmount).to.equal(1000000000);
    expect(splitBill.contributors).to.deep.equal([]);
  });


})