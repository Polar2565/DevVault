use anchor_lang::prelude::*;

declare_id!("DkevFFiPYgApvP5XRmnDETTGfs1e7Z74CMZ3QJrkxHFv");

#[program]
pub mod devvault {
    use super::*;

    pub fn iniciar_vault(ctx: Context<CrearVault>, alias: String) -> Result<()> {
        let authority_key = ctx.accounts.authority.key();

        ctx.accounts.vault.set_inner(AccountVault {
            authority: authority_key,
            alias,
            registros: Vec::new(),
        });

        Ok(())
    }

    pub fn registrar_entry(
        ctx: Context<ModificarVault>,
        etiqueta: String,
        puntuacion: u8,
    ) -> Result<()> {
        // Solo el dueño puede modificar
        require!(
            ctx.accounts.vault.authority == ctx.accounts.authority.key(),
            CodigoError::SinPermiso
        );

        require!(puntuacion <= 100, CodigoError::PuntuacionInvalida);

        // Evitar duplicados
        require!(
            !ctx.accounts
                .vault
                .registros
                .iter()
                .any(|r| r.etiqueta == etiqueta),
            CodigoError::RegistroDuplicado
        );

        ctx.accounts.vault.registros.push(Entry {
            etiqueta,
            puntuacion,
            activo: true,
        });

        Ok(())
    }

    pub fn remover_entry(ctx: Context<ModificarVault>, etiqueta: String) -> Result<()> {
        require!(
            ctx.accounts.vault.authority == ctx.accounts.authority.key(),
            CodigoError::SinPermiso
        );

        let registros = &mut ctx.accounts.vault.registros;

        let idx = registros
            .iter()
            .position(|r| r.etiqueta == etiqueta)
            .ok_or(CodigoError::RegistroNoExiste)?;

        registros.remove(idx);

        Ok(())
    }

    pub fn alternar_estado(ctx: Context<ModificarVault>, etiqueta: String) -> Result<()> {
        require!(
            ctx.accounts.vault.authority == ctx.accounts.authority.key(),
            CodigoError::SinPermiso
        );

        let registro = ctx
            .accounts
            .vault
            .registros
            .iter_mut()
            .find(|r| r.etiqueta == etiqueta)
            .ok_or(CodigoError::RegistroNoExiste)?;

        registro.activo = !registro.activo;

        Ok(())
    }

    pub fn ver_registros(ctx: Context<ModificarVault>) -> Result<()> {
        require!(
            ctx.accounts.vault.authority == ctx.accounts.authority.key(),
            CodigoError::SinPermiso
        );

        msg!("Registros actuales: {:#?}", ctx.accounts.vault.registros);
        Ok(())
    }
}

#[error_code]
pub enum CodigoError {
    #[msg("No autorizado")]
    SinPermiso,

    #[msg("Registro inexistente")]
    RegistroNoExiste,

    #[msg("Puntuación inválida (0-100)")]
    PuntuacionInvalida,

    #[msg("Registro duplicado")]
    RegistroDuplicado,
}

#[account]
#[derive(InitSpace)]
pub struct AccountVault {
    pub authority: Pubkey,

    #[max_len(32)]
    pub alias: String,

    #[max_len(25)]
    pub registros: Vec<Entry>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Entry {
    #[max_len(32)]
    pub etiqueta: String,
    pub puntuacion: u8,
    pub activo: bool,
}

#[derive(Accounts)]
pub struct CrearVault<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        payer = authority,
        space = AccountVault::INIT_SPACE + 8,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, AccountVault>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ModificarVault<'info> {
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [b"vault", authority.key().as_ref()],
        bump
    )]
    pub vault: Account<'info, AccountVault>,
}
