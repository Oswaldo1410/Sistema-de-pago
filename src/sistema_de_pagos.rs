use anchor_lang::prelude::*;

// Playground autogenerará y reemplazará este ID al hacer el build
declare_id!("11111111111111111111111111111111"); 

#[program]
pub mod payment_db {
    use super::*;

    pub fn process_payment(ctx: Context<ProcessPayment>, amount: u64, payment_id: String) -> Result<()> {
        
        // 1. Lógica para transferir el SOL entre billeteras
        let transfer_instruction = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.sender.key(),
            &ctx.accounts.receiver.key(),
            amount,
        );
        
        anchor_lang::solana_program::program::invoke(
            &transfer_instruction,
            &[
                ctx.accounts.sender.to_account_info(),
                ctx.accounts.receiver.to_account_info(),
            ],
        )?;

        // 2. Guardar el registro en nuestra "base de datos" (La cuenta PDA)
        let payment_record = &mut ctx.accounts.payment_record;
        payment_record.sender = *ctx.accounts.sender.key;
        payment_record.receiver = *ctx.accounts.receiver.key;
        payment_record.amount = amount;
        payment_record.payment_id = payment_id;
        
        // Obtenemos el tiempo actual de la red
        let clock = Clock::get()?;
        payment_record.timestamp = clock.unix_timestamp;

        msg!("Pago de {} lamports procesado y registrado con éxito.", amount);

        Ok(())
    }
}

// Aquí definimos las cuentas necesarias para la transacción
#[derive(Accounts)]
#[instruction(amount: u64, payment_id: String)]
pub struct ProcessPayment<'info> {
    #[account(mut)]
    pub sender: Signer<'info>, // Quien paga y firma

    /// CHECK: Es seguro porque solo le estamos enviando SOL, no leyendo datos.
    #[account(mut)]
    pub receiver: AccountInfo<'info>, // Quien recibe

    // Inicializamos la cuenta (el registro en la DB)
    #[account(
        init,
        payer = sender,
        // Calculamos el espacio: 8 (discriminador) + 32 (Pubkey) + 32 (Pubkey) + 8 (u64) + 4 (prefijo string) + 50 (max chars) + 8 (i64) = 142
        space = 142, 
        // Generamos la PDA basándonos en la palabra "payment", la llave del usuario y el ID del pago
        seeds = [b"payment", sender.key().as_ref(), payment_id.as_bytes()],
        bump
    )]
    pub payment_record: Account<'info, PaymentRecord>,

    pub system_program: Program<'info, System>,
}

// Esta es la estructura de tu "Tabla" en la base de datos
#[account]
pub struct PaymentRecord {
    pub sender: Pubkey,
    pub receiver: Pubkey,
    pub amount: u64,
    pub payment_id: String,
    pub timestamp: i64,
}