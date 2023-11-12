import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { expect } from "chai";
import { Zone } from "../target/types/zone";

describe("zone", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Zone as Program<Zone>;

  const counter = anchor.web3.Keypair.generate();

  it("Is initialized!", async () => {
    // Add your test here.
    await program.methods.initialize().accounts({ counter: counter.publicKey }).signers([counter]).rpc();

    const account = await program.account.counter.fetch(counter.publicKey);

    expect(account.count.toNumber() === 0);
  });

  it("Incremented the count", async () => {
    await program.methods.increment().accounts({ counter: counter.publicKey, user: provider.wallet.publicKey }).rpc();

    const account = await program.account.counter.fetch(counter.publicKey);

    expect(account.count.toNumber() === 1);
  })
});
