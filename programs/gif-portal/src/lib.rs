
use anchor_lang::prelude::*;

declare_id!("7J6KMrvYAtwtEBvU6qAzXJCNpQf48cQxWsEHYUV1N6f1");

#[program]
pub mod gif_portal {
    use super::*;
    pub fn create_account(ctx: Context<CreateAccount>) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.
        base_account.total_gifs = 0;
        Ok(())
    }
    pub fn add_gif(ctx: Context<AddGif>, link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let user = &ctx.accounts.user;
        let gif = Gif {
            link: link.clone(),
            total_upvotes: 0,
            user_address: *user.to_account_info().key
        };
        base_account.links.push(gif);
        base_account.total_gifs += 1;
        Ok(())
    }
    pub fn upvote(ctx: Context<UpvoteGif>, link: String) -> ProgramResult {
        let base_account = &mut ctx.accounts.base_account;
        let gif = base_account.links.iter_mut().find(|gif| gif.link == link);
        if let Some(mut gif) = gif {
            gif.total_upvotes += 1;
        }
        Ok(())
    }
    pub fn tip(ctx: Context<TipGif>, amount: u64) -> ProgramResult {
        let tx = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(), 
            &ctx.accounts.gif_author.key(), 
            amount
        );
        anchor_lang::solana_program::program::invoke(&tx, &[
            ctx.accounts.from.to_account_info(),
            ctx.accounts.gif_author.to_account_info()
        ])
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct CreateAccount<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}

#[derive(Accounts)]
pub struct UpvoteGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>
}


#[derive(Accounts)]
pub struct TipGif<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    #[account(mut)]
    pub gif_author: AccountInfo<'info>,
    system_program: Program<'info, System>
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Gif {
    pub link: String,
    pub user_address: Pubkey,
    pub total_upvotes: u32,
}

#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub links: Vec<Gif>
}



