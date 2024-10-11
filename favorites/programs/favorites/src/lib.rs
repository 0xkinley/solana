use anchor_lang::context::Context;
use anchor_lang::prelude::*;

declare_id!("9S2zkmKbBKt2DSwcvHexBZufQn4GtnTattrjVKq6bT6R");

pub const ANCHOR_DISCRIMINATOR_SIZE: usize = 8;

#[program]
pub mod favorites {

    use super::*;

    pub fn set_favorites(
        ctx: Context<SetFavorites>,
        number: u64,
        color: String,
        hobbies: Vec<String>,
    ) -> Result<()> {
        let user_public_key = ctx.accounts.user.key();

        msg!("Greetings From {}", ctx.program_id);

        msg!("User {}'s favorite number is {}, favorite color is {} and favorite hobbies are {:?}", user_public_key, number, color, hobbies);

        ctx.accounts.favorites.set_inner(Favorites {
            number,
            color,
            hobbies,
        });

        Ok(())
    }

    pub fn get_favorites(context: Context<GetFavorites>) -> Result<()> {
        let favorites = &context.accounts.phrase;

        msg!("User's favorite number is: {}", favorites.number);
        msg!("User's favorite color is: {}", favorites.color);
        msg!("User's hobbies are: {:?}", favorites.hobbies);

        Ok(())
    }
}

#[account]
#[derive(InitSpace)]
pub struct Favorites {
    pub number: u64,

    #[max_len(50)]
    pub color: String,

    #[max_len(5, 50)]
    pub hobbies: Vec<String>,
}

#[derive(Accounts)]
pub struct SetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init_if_needed,
        payer = user,
        space = ANCHOR_DISCRIMINATOR_SIZE + Favorites::INIT_SPACE,
        seeds = [b"Favorite", user.key().as_ref()],
        bump
    )]
    pub favorites: Account<'info, Favorites>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct GetFavorites<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        seeds = [b"Favorite", user.key().as_ref()],
        bump
    )]
    pub phrase: Account<'info, Favorites>,
}
