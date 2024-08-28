import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { NamasteSolana } from "../target/types/namaste_solana";

describe("namaste-solana", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.NamasteSolana as Program<NamasteSolana>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
