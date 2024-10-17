// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor'
import { Cluster, PublicKey } from '@solana/web3.js'
import TodolistIDL from '../target/idl/todolist.json'
import type { Todolist } from '../target/types/todolist'

// Re-export the generated IDL and type
export { Todolist, TodolistIDL }

// The programId is imported from the program IDL.
export const TODOLIST_PROGRAM_ID = new PublicKey(TodolistIDL.address)

// This is a helper function to get the Todolist Anchor program.
export function getTodolistProgram(provider: AnchorProvider) {
  return new Program(TodolistIDL as Todolist, provider)
}

// This is a helper function to get the program ID for the Todolist program depending on the cluster.
export function getTodolistProgramId(cluster: Cluster) {
  switch (cluster) {
    case 'devnet':
    case 'testnet':
      // This is the program ID for the Todolist program on devnet and testnet.
      return new PublicKey('CounNZdmsQmWh7uVngV9FXW2dZ6zAgbJyYsvBpqbykg')
    case 'mainnet-beta':
    default:
      return TODOLIST_PROGRAM_ID
  }
}
