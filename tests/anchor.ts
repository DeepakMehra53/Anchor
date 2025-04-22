import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Anchor } from "../target/types/anchor";
import { Keypair } from "@solana/web3.js";
import { program } from "@coral-xyz/anchor/dist/cjs/native/system";

describe("anchor", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchor as Program<Anchor>;

  const counterAccount = new Keypair();

  it("Is initialized!", async () => {
    // Add your test here.
    const transactionSiguature = await program.methods
      .initialize()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .signers([counterAccount])
      .rpc({ skipPreflight: true });

    const accountData = await program.account["counter"].fetch(
      counterAccount.publicKey
    );
    console.log(`Transaction Signture: ${transactionSiguature}`);
    console.log(`Count:${accountData.count}`)
  });

  it("Increment", async () => {
    // Add your test here.
    const transactionSiguature = await program.methods
      .increment()
      .accounts({
        counter: counterAccount.publicKey,
      })
      .rpc();

    const accountData = await program.account["counter"].fetch(
      counterAccount.publicKey
    );
    console.log(`Transaction Signture: ${transactionSiguature}`);
    console.log(`Count:${accountData.count}`)
  });
});




