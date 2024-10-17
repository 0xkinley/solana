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
  let provider;
  let todolistProgram: anchor.Program<Todolist>;


  beforeAll(async () => {
    context = await startAnchor("", [{ name: "todolist", programId: todolistAddress }], []);
    provider = new BankrunProvider(context);
    todolistProgram = new Program<Todolist>(
      IDL,
      provider
    );

    
  });

  it("Initialize ToDo List", async () => {

    // Initialize the ToDo list
    await todolistProgram.methods.initializeList("MyToDoList").rpc();

    let [listAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("MyToDoList"), context.payer.publicKey.toBuffer()],
      todolistAddress
    );

    let list = await todolistProgram.account.list.fetch(listAddress);


    console.log(list);

    // Assertions
    expect(list.listName).toEqual("MyToDoList");
    expect(list.taskCount).toEqual(0);
  });

  it("Add Task", async () => {
    // Derive the PDA for the ToDoList, ensure it's consistent with InitializeList
    let [listAddress] = PublicKey.findProgramAddressSync(
      [Buffer.from("MyToDoList"), context.payer.publicKey.toBuffer()],
      todolistAddress
    );
  
    // Call the addTask method and ensure correct accounts are passed
    await todolistProgram.methods.addTask("Go to gym").rpc();
  
    // Fetch the updated list account to verify the task was added
    let list = await todolistProgram.account.list.fetch(listAddress);
  
    console.log(list);
    expect(list.taskCount).toEqual(1);
    expect(list.tasks[0].description).toEqual("Go to gym");
  });
  

  

});
