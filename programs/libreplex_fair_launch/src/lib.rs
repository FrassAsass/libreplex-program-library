use anchor_lang::prelude::*;

pub mod instructions;
pub use instructions::*;
declare_id!("8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP");

pub mod state;
pub mod errors;

pub use state::*;

#[program]
pub mod libreplex_fair_launch {

    use super::*;


   pub fn deploy_legacy(
        ctx: Context<DeployLegacyCtx>,
    ) -> Result<()> {
        instructions::deploy_legacy::deploy(
            ctx
        )
    }   

    pub fn initialise(
        ctx: Context<InitialiseCtx>,
        input: InitialiseInput
    ) -> Result<()> {
        instructions::initialise::initialise(
            ctx,
            input
        )
    }   

    pub fn mint_legacy(
        ctx: Context<MintLegacyCtx>,
    ) -> Result<()> {
        instructions::mint_legacy::mint_legacy(
            ctx
        )
    }   


    pub fn swap_to_fungible(
        ctx: Context<SwapToFungibleCtx>
    ) -> Result<()> {
        instructions::swap_to_fungible::swap_to_fungible(
            ctx
        )
    }   


    pub fn swap_to_nonfungible(
        ctx: Context<SwapToNonFungibleCtx>
    ) -> Result<()> {
        instructions::swap_to_nonfungible::swap_to_nonfungible(
            ctx
        )
    }   




}
