# Zone

# Accounts
- Everything is an account
- Can store some SOL
- Unique 256-bit addresss 
- Can store arbitray data

```
{
    key: number, // The address of the account
    lamports: number, // Lamports currently held. 1 Lamport = 10E-9SOL
    data: Uint8Array, // Data stored in the account
    is_executable: boolean, // is this data a program?
    owner: PublicKey, // The program with write access
}
```

# Programs
- Smart contracts on Solana are called "programs"
- Special kind of account
- Data is eBPF bytecode
- Written in Rust, C/C++, Python
- Programs are stateless: they read & write data to other accounts. This allows programs to be executed in parallel
- Must be the owner of an account to modify
- Programs process instructions
- Programs can send instructions to other programs

# Program instructions

```
{
    program_id: number, // The program this instruction is for
    keys: Array<{ // Accounts involved in the instruction
        key: PublicKey,
        is_mutable: boolean,
        is_signer: boolean,
    }>,
    data: Uint8Array, // Action + args
}
```

## Resources
- [Programming on Solana - An Introduction](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/)
- [SOL dev](https://www.soldev.app/)

