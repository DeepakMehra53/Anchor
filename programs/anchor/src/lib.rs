use anchor_lang::prelude::*;

declare_id!("6XujACUJMDnN68Gk4iHpRCRAVeA8PfLZL2czW2SYyXdW");

#[program]
pub mod anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let counter = &ctx.accounts.counter;
        msg!("Counter account create! Current count:{}",counter.count);
        Ok(())
    }
    pub fn increment(ctx:Context<Increment> )-> Result<()>{
        let counter = &mut ctx.accounts.counter;
        msg!("Pervious counter:{}",counter.count);

        counter.count = counter.count.checked_add(1).unwrap();
        msg!("Counter incremented! Current count:{}",counter.count);
        Ok(())

    }
}

#[derive(Accounts)]
pub struct Initialize<'info>{
    /* What does this do 
        we are running the initilize funtion you need to pass 4 thing
        1st:account of the user that actually paying for the transaction so when you're
        create an account you need to pay for that account, who's going to be paying 
        for this this is going to be the guy called user,so this is the account that's signing 
        the transaction and we will be paying for the other account to be create what the other 
        account

        2nd: second valus not really a value ,but more a declaration , we are declaraing ,
        hey we expect a counter account and if this counter doesn't exist I want you to create 
        it and this is some anchor magic that's happening here so anchor is going to do the work
        of creating managing and allocation the space for all of the accounts itself ,so it's going
        to do it's going to write code by itself , we are telling anchor hey counter account needs
        to be created if it doesn't already exist and it should initilized so again we'er telling 
        it hey initialize the account this is going to be the user this is going to be the payer 
        the person the person that's going to pay for this is going to be user ,and this is amount 
        of space we account we need for this account so whenever we create an account in solana we 
        need to declare how much space we're taking up because that's how much Sol we have to pay for 
        its usage based costs

    */
    #[account(mut)]
    pub user:Signer<'info>,//specifies we are creating this account
    //The counter account being create and initialized in the instruction
    #[account(
        init,       //Specifies we are creating this account
        payer=user, //Specifies account paying for the creating of the account
        space=8+8   //Space allocating to the new account (8 byte discriminator +8 byte for u64)
    )]
    pub  counter:Account<'info,Counter>,        //Specifies account is 'Counter ' type
    pub system_program: Program<'info,System>   //Specifies account must be system Program
}





//Account requires by the increment instruction
#[derive(Accounts)]
pub struct Increment<'info>{
    #[account(mut)] //specify account is mutable because we are updating its data
    pub counter: Account<'info,Counter>, //specify account is "Counter" type
}


//Define  structure of Counter account
#[account]
pub struct Counter{
    pub count :u64,
}