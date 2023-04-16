// Import necessary modules
use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::{Clock, UnixTimestamp};
use anchor_lang::pyth::account_info::{FromAccounts, PythAccountInfo};
use anchor_lang::pyth::{self, PriceAccount};

// Define the PythPrice struct
#[account]
pub struct PythPrice {
    pub price: f64,                 // Price of SOL at midnight UTC
    pub epoch: u64,                 // Epoch of the price account
}

// Define the get_pyth_price function
#[program]
pub fn get_pyth_price(
    ctx: Context<GetPythPrice>,
    #[account(init)] pyth_price: Account<PythPrice>,
) -> ProgramResult {
    // Load the Pyth price account
    let pyth_account_info = PythAccountInfo::from_accounts(ctx.accounts.clone())?;
    let price_account = PriceAccount::load(pyth_account_info.price_account().key, ctx.accounts.to_account_info().clone())?;

    // Get the current Unix timestamp
    let clock = Clock::get()?;
    let current_timestamp = clock.unix_timestamp as u64;

    // Get the Unix timestamp of the next midnight UTC
    let midnight_timestamp = next_midnight_utc(clock.unix_timestamp);

    // Ensure that the Pyth price account has updated data at midnight UTC
    if price_account.product_account().key != pyth_account_info.product_account().key {
        return Err(ErrorCode::InvalidPythAccount.into());
    }
    if price_account.next_slot() > midnight_timestamp {
        return Err(ErrorCode::NoMidnightPrice.into());
    }

    // Get the price of SOL at midnight UTC
    let price = price_account.price()?;

    // Save the PythPrice account
    pyth_price.price = price.0;
    pyth_price.epoch = price.1;
    pyth_price.serialize(&mut *ctx.accounts.pyth_price.to_account_info().data.borrow_mut())?;

    Ok(())
}

// Define the GetPythPrice struct
#[derive(Accounts)]
pub struct GetPythPrice<'info> {
    #[account(
        init,
        payer = user,
        space = PythPrice::get_packed_len(),
    )]
    pub pyth_price: Account<'info, PythPrice>,
    #[account(signer)]
    pub user: AccountInfo<'info>,
    #[account(constraint = clock.unix_timestamp <= next_midnight_utc(clock.unix_timestamp))]
    pub clock: AccountInfo<'info>,
    #[account(
        constraint = pyth_account.product_account() == pyth_price_product.key(),
        constraint = price_account.next_slot() <= next_midnight_utc(clock.unix_timestamp),
    )]
    pub pyth_account: AccountInfo<'info>,
    #[account(
        constraint = pyth_account_info.is_initialized(),
        constraint = price_account.is_initialized(),
    )]
    pub pyth_account_info: PythAccountInfo<'info>,
    #[account(
        constraint = mint.key() == token::ID,
        constraint = to_account.owner == token::ID,
    )]
    pub mint: AccountInfo<'info>,
    #[account(mut, constraint = to_account.owner == user.key())]
    pub to_account: AccountInfo<'info>,
    #[account(
        constraint = pyth_price_product.key() == pyth_account.product_account(),
        constraint = price_account.product_account() == pyth_account.product_account(),
    )]
    pub pyth_price_product: AccountInfo<'info>,
    #[account(
        constraint = pyth_price_price.key() == py
