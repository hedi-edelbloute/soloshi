import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Soloshi } from "../target/types/soloshi";
import { Keypair, PublicKey, SystemProgram } from "@solana/web3.js";
import {
  getOrCreateAssociatedTokenAccount,
  mintTo,
  createMint,
} from "@solana/spl-token";

describe("soloshi", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.soloshi as Program<Soloshi>;

  const petKeypair = Keypair.generate();
  let mint: PublicKey;
  let userTokenAccount: PublicKey;
  let petTokenAccount: PublicKey;

  it("Initializes pet and feeds it", async () => {
    console.log("Creating SPL token mint...");
    mint = await createMint(
      provider.connection,
      provider.wallet.payer,
      provider.wallet.publicKey,
      null,
      0
    );

    console.log("Creating user token account and minting tokens...");
    userTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        provider.wallet.payer,
        mint,
        provider.wallet.publicKey
      )
    ).address;

    console.log("Minting tokens to user token account...");
    await mintTo(
      provider.connection,
      provider.wallet.payer,
      mint,
      userTokenAccount,
      provider.wallet.payer,
      10
    );

    console.log("User token account:", userTokenAccount.toBase58());
    petTokenAccount = (
      await getOrCreateAssociatedTokenAccount(
        provider.connection,
        provider.wallet.payer,
        mint,
        petKeypair.publicKey
      )
    ).address;

    console.log("Pet token account:", petTokenAccount.toBase58());
    await program.methods
      .initialize()
      .accounts({
        pet: petKeypair.publicKey,
        owner: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([petKeypair])
      .rpc();

    console.log("Pet initialized successfully.");
    await program.methods
      .feedPet(new anchor.BN(5))
      .accounts({
        pet: petKeypair.publicKey,
        owner: provider.wallet.publicKey,
        userTokenAccount,
        petTokenAccount,
        tokenMint: mint,
        tokenProgram: anchor.utils.token.TOKEN_PROGRAM_ID,
      })
      .rpc();

    console.log("Pet fed successfully.");
  });
});
