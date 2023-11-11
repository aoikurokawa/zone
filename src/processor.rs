use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke_signed,
    program_error::ProgramError,
    program_pack::IsInitialized,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{
    error::ReviewError,
    instruction::MovieInstruction,
    state::{MovieAccountState, MovieComment, MovieCommentCounter},
};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = MovieInstruction::unpack(instruction_data)?;

    match instruction {
        MovieInstruction::AddMovieReview {
            title,
            rating,
            description,
        } => add_movie_review(program_id, accounts, title, rating, description),
        MovieInstruction::UpdateMovieReview {
            title,
            rating,
            description,
        } => update_movie_review(program_id, accounts, title, rating, description),
        MovieInstruction::AddComment { comment } => add_comment(program_id, accounts, comment),
    }
}

pub fn add_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Adding movie review...");
    msg!("Title: {}", title);
    msg!("Rating: {}", rating);
    msg!("Description: {}", description);

    let account_info_iter = &mut accounts.iter();

    let initializer = next_account_info(account_info_iter)?;
    let pda_account = next_account_info(account_info_iter)?;
    let pda_counter = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // account for movie
    let (pda, bump_seed) =
        Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes()], program_id);

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    if rating < 1 || rating > 5 {
        msg!("Rating cannot be higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    let account_len = 1000_usize;
    let total_len = MovieAccountState::get_account_size(&title, &description);
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_account.key,
            rent_lamports,
            account_len.try_into().unwrap(),
            program_id,
        ),
        &[
            initializer.clone(),
            pda_account.clone(),
            system_program.clone(),
        ],
        &[&[initializer.key.as_ref(), title.as_bytes(), &[bump_seed]]],
    )?;

    msg!("unpacking state account");
    let mut account_data: MovieAccountState =
        my_try_from_slice_unchecked(&pda_account.data.borrow())?;
    msg!("borrowed account data");

    msg!("checking if movie account is already initialized");
    if account_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    account_data.discriminator = MovieAccountState::DISCRIMINATOR.to_string();
    account_data.reviewer = *initializer.key;
    account_data.title = title;
    account_data.rating = rating;
    account_data.description = description;
    account_data.is_initialized = true;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.try_borrow_mut_data()?[..])?;
    msg!("state account serialized");

    // account for comment counter
    msg!("creating comment counter");
    let rent = Rent::get()?;
    let counter_rent_lamports = rent.minimum_balance(MovieCommentCounter::SIZE);

    let (counter, counter_bump) =
        Pubkey::find_program_address(&[pda.as_ref(), "comment".as_bytes()], program_id);
    if counter != *pda_counter.key {
        msg!("Invalid seeds for PDA");
        return Err(ProgramError::InvalidArgument);
    }

    invoke_signed(
        &system_instruction::create_account(
            initializer.key,
            pda_counter.key,
            counter_rent_lamports,
            MovieCommentCounter::SIZE as u64,
            program_id,
        ),
        &[
            initializer.clone(),
            pda_counter.clone(),
            system_program.clone(),
        ],
        &[&[pda.as_ref(), "comment".as_bytes(), &[counter_bump]]],
    )?;
    msg!("comment counter created");

    let mut counter_data: MovieCommentCounter =
        my_try_from_slice_unchecked(&pda_counter.try_borrow_mut_data()?[..])?;

    msg!("checking if counter is already initialized");
    if counter_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    counter_data.discriminator = MovieCommentCounter::DISCRIMINATOR.to_string();
    counter_data.counter = 0;
    counter_data.is_initialized = true;
    msg!("comment count: {}", counter_data.counter);
    counter_data.serialize(&mut &mut pda_counter.try_borrow_mut_data()?[..])?;

    Ok(())
}

pub fn update_movie_review(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    rating: u8,
    description: String,
) -> ProgramResult {
    msg!("Updating movie review...");

    // Get account iterator
    let mut account_info_iter = accounts.iter();

    let initializer = next_account_info(&mut account_info_iter)?;
    let pda_account = next_account_info(&mut account_info_iter)?;

    if pda_account.owner != program_id {
        return Err(ProgramError::InvalidAccountOwner);
    }

    if !initializer.is_signer {
        msg!("Missing required signature");
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, _bump_seed) =
        Pubkey::find_program_address(&[initializer.key.as_ref(), title.as_bytes()], program_id);

    if pda != *pda_account.key {
        msg!("Invalid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    if rating < 1 || rating > 5 {
        msg!("Rating cannot be higher than 5");
        return Err(ReviewError::InvalidRating.into());
    }

    msg!("unpacking state account");
    let mut account_data: MovieAccountState =
        my_try_from_slice_unchecked(&pda_account.data.borrow())?;
    msg!("borrowed account data");

    msg!("checking if movie account is already initialized");
    if account_data.is_initialized {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    let total_len: usize = MovieAccountState::get_account_size(&title, &description);
    if total_len > 1000 {
        msg!("Data length is larger than 1000 bytes");
        return Err(ReviewError::InvalidDataLength.into());
    }

    account_data.rating = rating;
    account_data.description = description;

    msg!("serializing account");
    account_data.serialize(&mut &mut pda_account.try_borrow_mut_data()?[..])?;
    msg!("state account serialized");

    Ok(())
}

pub fn add_comment(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    comment: String,
) -> ProgramResult {
    msg!("Adding Comment...");
    msg!("Comment: {}", comment);

    let mut account_info_iter = accounts.iter();

    let commenter = next_account_info(&mut account_info_iter)?;
    let pda_review = next_account_info(&mut account_info_iter)?;
    let pda_counter = next_account_info(&mut account_info_iter)?;
    let pda_comment = next_account_info(&mut account_info_iter)?;
    let system_program = next_account_info(&mut account_info_iter)?;

    let mut counter_data: MovieCommentCounter =
        my_try_from_slice_unchecked(&pda_counter.try_borrow_mut_data()?[..])?;

    let account_len = MovieComment::get_account_size(&comment);

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(account_len);

    let (pda, bump_seed) = Pubkey::find_program_address(
        &[
            pda_review.key.as_ref(),
            counter_data.counter.to_be_bytes().as_ref(),
        ],
        program_id,
    );
    if pda != *pda_comment.key {
        msg!("Invlaid seeds for PDA");
        return Err(ReviewError::InvalidPDA.into());
    }

    invoke_signed(
        &system_instruction::create_account(
            commenter.key,
            pda_comment.key,
            rent_lamports,
            account_len as u64,
            program_id,
        ),
        &[
            commenter.clone(),
            pda_comment.clone(),
            system_program.clone(),
        ],
        &[&[
            pda_review.key.as_ref(),
            counter_data.counter.to_be_bytes().as_ref(),
            &[bump_seed],
        ]],
    )?;

    msg!("created comment account");

    let mut comment_data: MovieComment =
        my_try_from_slice_unchecked(&pda_comment.try_borrow_mut_data()?[..])?;

    msg!("checking if comment account is already initialized");
    if comment_data.is_initialized() {
        msg!("Account already initialized");
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    comment_data.discriminator = MovieComment::DISCRIMINATOR.to_string();
    comment_data.review = *pda_review.key;
    comment_data.commenter = *commenter.key;
    comment_data.comment = comment;
    comment_data.is_initialized = true;
    comment_data.serialize(&mut &mut pda_comment.try_borrow_mut_data()?[..])?;

    msg!("Comment Count: {}", counter_data.counter);
    counter_data.counter += 1;
    counter_data.serialize(&mut &mut pda_counter.try_borrow_mut_data()?[..])?;

    Ok(())
}

pub fn my_try_from_slice_unchecked<T: borsh::BorshDeserialize>(
    data: &[u8],
) -> Result<T, ProgramError> {
    let mut data = data;
    match T::deserialize(&mut data) {
        Ok(result) => Ok(result),
        Err(_) => Err(ProgramError::InvalidInstructionData),
    }
}
