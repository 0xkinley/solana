#![allow(clippy::result_large_err)]

use anchor_lang::prelude::*;

declare_id!("6u7Wzgps8X8Qjd5AaqaF5mpKdfZzSfNt2MaPjATf2Z6Y");

#[program]
pub mod todolist {
    use super::*;

    pub fn initialize_list(ctx: Context<InitializeList>, list_name: String) -> Result<()>{

      let todolist = &mut ctx.accounts.todolist;
      todolist.owner = *ctx.accounts.owner.key;
      todolist.list_name = list_name;
      todolist.task_count = 0;
      
      Ok(())
    }

    pub fn add_task(ctx: Context<AddTasks>, task: String) -> Result<()> {

      let todolist  = &mut ctx.accounts.todolist;
      let task_state = Task{
        description: task,
        is_completed: false,
      };

      todolist.tasks.push(task_state);
      todolist.task_count += 1;

      Ok(())
    }

    pub fn remove_task(ctx: Context<RemoveTask>, task_index: u32) -> Result<()> {

      let todolist = &mut ctx.accounts.todolist;

      if(task_index as usize) < todolist.tasks.len() {

        todolist.tasks.remove(task_index as usize);
        todolist.task_count -=1;

      } else {

        return Err(ErrorCode::TaskNotFound.into());

      }

      Ok(())
    }

    pub fn complete_task(ctx: Context<CompleteTask>, task_index: u32) -> Result<()> {

      let todolist = &mut ctx.accounts.todolist;

      if(task_index as usize) < todolist.tasks.len() {

        let task = &mut todolist.tasks[task_index as usize];
            task.is_completed = true;

      } else {

        return Err(ErrorCode::TaskNotFound.into());

      }

      Ok(())
    }

  
  
}

#[account]
pub struct Task{

  pub description: String,

  pub is_completed: bool,
}

#[account]
pub struct List {

  pub owner: Pubkey,
  
  pub list_name: String,

  pub tasks: Vec<Task>,

  pub task_count: u32,
}

impl List {
  pub const MAX_TASKS: usize = 10; // Adjust this based on your requirements
  pub const MAX_LEN_LIST_NAME: usize = 100; // Max length of list_name

  pub fn space() -> usize {
      8 + 32 + // Discriminator + Pubkey for owner
      4 + Self::MAX_LEN_LIST_NAME + // List name (4 bytes for length + max 100 bytes)
      4 + (Self::MAX_TASKS * 285) + // Vec<Task>: 4 bytes for length prefix + 50 tasks of 285 bytes each
      4 // u32 task_count
  }
}

#[derive(Accounts)]
#[instruction(list_name: String)]
pub struct InitializeList<'info> {

  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    init,
    payer = owner,
    space = List::space(),
    seeds = [list_name.as_bytes(), owner.key().as_ref()],
    bump
  )]
  pub todolist: Account<'info, List>,

  pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(list_name: String)]
pub struct AddTasks<'info> {
  #[account(mut)]
  pub owner: Signer<'info>,

  #[account(
    mut, 
    seeds = [list_name.as_bytes(), owner.key().as_ref()], 
    bump)]
    pub todolist: Account<'info, List>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(list_name: String)]
pub struct RemoveTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(mut, 
      seeds = [list_name.as_bytes(), owner.key().as_ref()], 
      bump)]
    pub todolist: Account<'info, List>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(list_name: String)]
pub struct CompleteTask<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
      mut, 
      seeds = [list_name.as_bytes(), owner.key().as_ref()], 
      bump)]
    pub todolist: Account<'info, List>,

    pub system_program: Program<'info, System>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Task not found in the ToDoList.")]
    TaskNotFound,
}