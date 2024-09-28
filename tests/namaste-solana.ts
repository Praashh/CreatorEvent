import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
import { expect } from "chai";
import { NfTickets } from "../target/types/nf_tickets";
import { fetchCollectionV1, fetchAssetV1 } from "@metaplex-foundation/mpl-core";
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults";
import { mplCore } from "@metaplex-foundation/mpl-core";
import { publicKey } from "@metaplex-foundation/umi";

// Use the RPC endpoint of your choice.
const umi = createUmi("http://127.0.0.1:8899").use(mplCore());

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

  it("Initializes the manager", async () => {
    const [managerPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("manager"), provider.wallet.publicKey.toBuffer()],
      program.programId
    );

    const tx = await program.methods.setupManager().accounts({}).rpc();

    // Fetch the manager account to verify initialization
    const managerAccount = await program.account.manager.fetch(managerPda);
    console.log(managerAccount);
  });

  it("Creates an event", async () => {
    // Generate a new keypair for the event
    const eventKeypair = anchor.web3.Keypair.generate();

    // Define event details
    const eventArgs = {
      name: "Test Event",
      category: "Music",
      uri: "https://example.com/event",
      city: "Test City",
      venue: "Test Venue",
      artist: "Test Artist",
      date: "2024-10-01",
      time: "20:00",
      capacity: 1,
      isTicketTransferable: true,
    };

    const tx = await program.methods
      .createEvent(eventArgs)
      .accounts({
        event: eventKeypair.publicKey,
      })
      .signers([eventKeypair])
      .rpc();

    console.log("Transaction signature:", tx);

    // Wait for the transaction to be confirmed
    await provider.connection.confirmTransaction(tx);

    // Function to fetch collection with retry
    const fetchCollectionWithRetry = async (retries = 50, delay = 2000) => {
      for (let i = 0; i < retries; i++) {
        try {
          const collectionId = publicKey(eventKeypair.publicKey.toBase58());
          const collection = await fetchCollectionV1(umi, collectionId);
          console.log("Fetched collection:", collection);
          return collection;
        } catch (error) {
          if (i === retries - 1) throw error;
          console.log(`Attempt ${i + 1} failed, retrying in ${delay}ms...`);
          await new Promise((resolve) => setTimeout(resolve, delay));
        }
      }
    };

    // Fetch the collection using the retry mechanism
    const collection = await fetchCollectionWithRetry();

    // Add assertions to verify the collection details
    expect(collection.name).to.equal(eventArgs.name);
    expect(collection.uri).to.equal(eventArgs.uri);

    //   const ticketKeypair = anchor.web3.Keypair.generate();
    //   const venueAuthority = anchor.web3.Keypair.generate().publicKey;

    //   const ticketArgs = {
    //     name: "Test Ticket",
    //     uri: "https://example.com/ticket",
    //     price: new anchor.BN(1000000), // 1 SOL in lamports
    //     venueAuthority: venueAuthority,
    //     screen: "Screen 1",
    //     row: "A",
    //     seat: "1",
    //   };

    //   const [treasuryPda] = anchor.web3.PublicKey.findProgramAddressSync(
    //     [Buffer.from("treasury"), platformPda.toBuffer()],
    //     program.programId
    //   );

    //   //const treasuryAccount = await program.account.platform.fetch(treasuryPda);

    //   const ticketTx = await program.methods
    //   .createTicket(ticketArgs)
    //   .accounts({
    //     signer: provider.wallet.publicKey,
    //     payer: provider.wallet.publicKey,
    //     manager: managerPda,
    //     platform: platformPda,
    //     event: eventKeypair.publicKey,
    //     ticket: ticketKeypair.publicKey,
    //     treasury: treasuryPda,
    //     systemProgram: anchor.web3.SystemProgram.programId,
    //     mplCoreProgram: new anchor.web3.PublicKey("Core1111111111111111111111111111111111111"),
    //   })
    //   .signers([ticketKeypair])
    //   .rpc();

    // console.log("Ticket creation transaction signature:", ticketTx);

    // // Wait for the transaction to be confirmed
    // await provider.connection.confirmTransaction(ticketTx);

    // // Function to fetch ticket with retry
    // const fetchTicketWithRetry = async (retries = 50, delay = 2000) => {
    //   for (let i = 0; i < retries; i++) {
    //     try {
    //       const ticketId = publicKey(ticketKeypair.publicKey.toBase58());
    //       const ticket = await fetchAssetV1(umi, ticketId);
    //       console.log("Fetched ticket:", ticket);
    //       return ticket;
    //     } catch (error) {
    //       if (i === retries - 1) throw error;
    //       console.log(`Attempt ${i + 1} failed, retrying in ${delay}ms...`);
    //       await new Promise((resolve) => setTimeout(resolve, delay));
    //     }
    //   }
    // };

    // // Fetch and verify ticket details
    // const ticket = await fetchTicketWithRetry();

    // expect(ticket.name).to.equal(ticketArgs.name);
    // expect(ticket.uri).to.equal(ticketArgs.uri)
  });

  it("Scans a ticket", async () => {
    // Test implementation for scanning a ticket
  });
});
