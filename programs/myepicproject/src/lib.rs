use anchor_lang::prelude::*;

declare_id!("GiARCEmixpiQAxAfDRWTGS6MX26MqQ4gVppVt38YaaoW");

#[program]
pub mod myepicproject {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> Result <()> {
    // Obtem a referência da conta
    let base_account = &mut ctx.accounts.base_account;
    // Inicializa o total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  // Outra função uhul!
  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> Result <()> {
    // Obtem a referencia para a conta e incrementa total_gifs.
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
    };

    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
  }
}

// Anexa algumas variaveis ao contexto do StartStuffOff.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Especifica que dados queremos no Contexto AddGif
// Obtendo um controle sobre o fluxo das coisas 😊?
#[derive(Accounts)]
pub struct AddGif<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
  pub gif_link: String,
  pub user_address: Pubkey,
}

// Conta para a Solana o que queremos guardar nessa conta.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}
