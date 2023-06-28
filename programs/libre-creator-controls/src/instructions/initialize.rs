use anchor_lang::{prelude::*};
use librecreator::{Creator, instructions::UpdateInput};

use crate::state::{Phase, CreatorController};


#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct InitializeInput {
    pub phases: Vec<Phase>,
    pub seed: Pubkey,
}

#[derive(Accounts)]
#[instruction(input: InitializeInput)]
pub struct Initialize<'info> {
    pub update_authority: Signer<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    #[account(init, payer = payer, space = CreatorController::size_for_input(&input.phases), seeds = [input.seed.as_ref()], bump)]
    pub creator_controller: Account<'info, CreatorController>,

    pub creator: Account<'info, Creator>,

    pub system_program: Program<'info, System>,

    /// CHECK: Only check the address
    #[account(address = librecreator::id())]
    pub libreplex_creator_program: AccountInfo<'info>,
}

pub fn handler(ctx: Context<Initialize>, input: InitializeInput) -> Result<()> {
    let controller = &mut ctx.accounts.creator_controller;

    controller.phases = input.phases;
    controller.update_authority = ctx.accounts.update_authority.key();
    controller.seed = input.seed;
    controller.bump = *ctx.bumps.get("creator_controller").unwrap();


    let cpi_ctx = CpiContext::new(ctx.accounts.libreplex_creator_program.to_account_info(), librecreator::cpi::accounts::UpdateCreator {
        creator: ctx.accounts.creator.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        update_authority: ctx.accounts.update_authority.to_account_info(),
    });

    librecreator::cpi::update(cpi_ctx, UpdateInput {
        mint_authority: controller.key(),
    })
}
