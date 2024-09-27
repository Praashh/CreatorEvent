import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import { NfTickets } from "../target/types/nf_tickets";

describe("nf-tickets", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.NfTickets as Program<NfTickets>;

  const platformName = "TestPlatform";
  const fee = 250; // 2.5%

  const [platformPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("platform"), Buffer.from(platformName)],
    program.programId
  );

  it("Initializes the platform", async () => {
    const tx = await program.methods
      .initialize(platformName, fee)
      .accounts({
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .rpc();

    // Fetch the platform account to verify initialization
    const platformAccount = await program.account.platform.fetch(platformPda);

    // Assert the correct values were set
    expect(platformAccount.admin.toString()).to.equal(
      provider.wallet.publicKey.toString()
    );
    expect(platformAccount.fee).to.equal(fee);
    expect(platformAccount.platformName).to.equal(platformName);
    expect(platformAccount.bump).to.be.greaterThan(0);
    expect(platformAccount.treasuryBump).to.be.greaterThan(0);
    expect(platformAccount.rewardsBump).to.be.greaterThan(0);
  });

  it("Creates a ticket", async () => {});

  it("Scans a ticket", async () => {});
});
