import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { EscrowMarketplaceProgram } from "../target/types/escrow_marketplace_program";

describe("escrow-marketplace-program", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.EscrowMarketplaceProgram as Program<EscrowMarketplaceProgram>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
