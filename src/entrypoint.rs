// This brings the required into scope.
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

use crate::processor::Processor;
//This declares process_instruction as the entrypoint of the program.
entrypoint!(process_instruction);

/* Entry point takes 3 arguments; the program id of the executing program, instruction data
which is anything and accounts which programs are stored in.
*/
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    Processor::process(program_id, accounts, instruction_data)
}