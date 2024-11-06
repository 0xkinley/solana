import { ProgramTestContext, startAnchor } from "solana-bankrun";
import { BankrunProvider } from "anchor-bankrun";
import * as anchor from '@coral-xyz/anchor';
import { Program } from '@coral-xyz/anchor';
import { PublicKey, SystemProgram } from '@solana/web3.js';
import { Todolist } from '../target/types/todolist';

const IDL = require("../target/idl/todolist.json");
const todolistAddress = new PublicKey("6u7Wzgps8X8Qjd5AaqaF5mpKdfZzSfNt2MaPjATf2Z6Y");

describe('todolist', () => {
  let context: ProgramTestContext;
  let provider: BankrunProvider;
  let todolistProgram: anchor.Program<Todolist>;
  let listAddress: PublicKey;
  const LIST_NAME = "MyToDoList";

  beforeAll(async () => {
    context = await startAnchor("", [{ name: "todolist", programId: todolistAddress }], []);
    provider = new BankrunProvider(context);
    todolistProgram = new Program<Todolist>(
      IDL,
      provider
    );

    // Pre-calculate the list address
    [listAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from(LIST_NAME), context.payer.publicKey.toBuffer()],
      todolistProgram.programId
    );
  });

  it("Initialize ToDo List", async () => {
    await todolistProgram.methods
      .initializeList(LIST_NAME)
      .accounts({
        owner: context.payer.publicKey,
      })
      .rpc();

    let list = await todolistProgram.account.list.fetch(listAddress);
    console.log(list);

    expect(list.listName).toEqual(LIST_NAME);
    expect(list.taskCount).toEqual(0);
  });

  it("Add Task", async () => {
    await todolistProgram.methods
      .addTask(LIST_NAME, "Go to gym")
      .accounts({
        owner: context.payer.publicKey,
        todolist: listAddress,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let list = await todolistProgram.account.list.fetch(listAddress);
    console.log("After adding task:", list);
    
    expect(list.taskCount).toEqual(1);
    expect(list.tasks[0].description).toEqual("Go to gym");
  });

  it("Complete Task", async () => {
    await todolistProgram.methods
      .completeTask(LIST_NAME, 0)
      .accounts({
        owner: context.payer.publicKey,
        todolist: listAddress,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let list = await todolistProgram.account.list.fetch(listAddress);
    expect(list.tasks[0].isCompleted).toEqual(true);
  });

  it("Remove Task", async () => {
    await todolistProgram.methods
      .removeTask(LIST_NAME, 0)
      .accounts({
        owner: context.payer.publicKey,
        todolist: listAddress,
        systemProgram: SystemProgram.programId,
      })
      .rpc();

    let list = await todolistProgram.account.list.fetch(listAddress);
    expect(list.taskCount).toEqual(0);
    expect(list.tasks).toHaveLength(0);
  });
});