use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke_signed,
    program_error::ProgramError, pubkey::Pubkey, system_instruction::transfer,
};

pub fn close_account_and_transfer_rent<'a>(
    _program_id: &Pubkey,
    account_to_close_info: &AccountInfo<'a>,
    recipient_info: &AccountInfo<'a>,
    rent_sysvar_info: &AccountInfo<'a>,
) -> ProgramResult {
    // Get account to be closed
    let account_to_close = account_to_close_info.key;

    // Get recipient account for rent refund
    let recipient = recipient_info.key;

    // Get account rent and balance
    let balance = **account_to_close_info
        .try_borrow_lamports()
        .map_err(|_| ProgramError::AccountBorrowFailed)?;

    // Check if account has rent outstanding
    if balance > 0 {
        // Calculate amount to be refunded
        let rent_refund_amount = balance;

        // Transfer rent refund to recipient account
        invoke_signed(
            &transfer(account_to_close, recipient, rent_refund_amount),
            &[
                account_to_close_info.clone(),
                recipient_info.clone(),
                rent_sysvar_info.clone(),
            ],
            &[&[b"rent-refund"]],
        )?;
    }

    Ok(())
}
