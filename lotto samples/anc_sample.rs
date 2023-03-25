use anchor_lang::prelude::*;

declare_id!("FcGD1W1PZKQ5NRpNRZuPYYwGkCFsia1hdPtudAkMa458");

#[program]
pub mod myepicproject {
    use super::*;

    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result<()> {
        // Get a reference to the account.
        let base_account = &mut ctx.accounts.base_account;
        // Initialize total_gifs.

        base_account.total_users = 0;
        base_account.total_duel = 0;
        Ok(())
    }
    pub fn add_player(ctx: Context<AddUser>, another_value: String) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;
        let user = &mut ctx.accounts.user;

        // Build the struct.
        let item = ItemStruct {
            user_address: *user.to_account_info().key,
            another_value: another_value.to_string(),
        };

        // Add it to the gif_list vector.
        base_account.user_list.push(item);

        base_account.total_users += 1;
        if base_account.total_users % 2 == 0 && base_account.total_users != 0 {
            base_account.total_duel += 1;

            // Build the struct.
            let item = ScoreStruct {
                winner: base_account.user_list[0].user_address,
                loser: base_account.user_list[1].user_address,
                duel: base_account.total_duel,
            };
            base_account.score_list.push(item);
            base_account.user_list.pop();
            base_account.user_list.pop();
        }

        Ok(())
    }
    pub fn del_user(ctx: Context<AddUser>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct.

        // Add it to the gif_list vector.
        base_account.user_list.pop();
        base_account.total_users -= 1;
        Ok(())
    }
    pub fn del_score(ctx: Context<AddUser>) -> Result<()> {
        let base_account = &mut ctx.accounts.base_account;

        // Build the struct.

        // Add it to the gif_list vector.
        base_account.score_list.pop();
        base_account.total_users -= 1;
        Ok(())
    }
}

// Attach certain variables to the StartStuffOff context.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 10000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}
// Add the signer who calls the AddGif method to the struct so that we can save it
#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}
// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub user_address: Pubkey,
    pub another_value: String,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ScoreStruct {
    pub winner: Pubkey,
    pub duel: u64,
    pub loser: Pubkey,
}

#[account]
pub struct BaseAccount {
    // Attach a Vector of type ItemStruct to the account.
    pub user_list: Vec<ItemStruct>,
    pub total_users: u64,
    pub score_list: Vec<ScoreStruct>,
    pub total_duel: u64,
}