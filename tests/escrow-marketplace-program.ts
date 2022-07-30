import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { PublicKey, SystemProgram } from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";
import { EscrowMarketplaceProgram } from "../target/types/escrow_marketplace_program";
import { assert } from "chai";
import { BN } from "bn.js";

describe("escrow-marketplace-program", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace
    .EscrowMarketplaceProgram as Program<EscrowMarketplaceProgram>;

  let nftMint: PublicKey = null;

  let sellerTokenAccount: PublicKey = null;
  let buyerTokenAccount: PublicKey = null;

  let escrowTokenAccount: PublicKey = null;

  let escrowInfoAccount: PublicKey = null;
  let escrowInfoAccountBump: number = null;

  const sellerNftTokenAmount = 1;
  const sellerListingPrice = 1e9;

  const seller = anchor.web3.Keypair.generate();
  const buyer = anchor.web3.Keypair.generate();

  it("initializes mint and token accounts", async () => {
    // Add your test here.

    const airdropSellerSig = await provider.connection.requestAirdrop(
      seller.publicKey,
      2e9
    );
    const latestSellerBlockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestSellerBlockhash.blockhash,
      lastValidBlockHeight: latestSellerBlockhash.lastValidBlockHeight,
      signature: airdropSellerSig,
    });

    const airdropBuyerSig = await provider.connection.requestAirdrop(
      buyer.publicKey,
      2e9
    );
    const latestBuyerBlockhash = await provider.connection.getLatestBlockhash();

    await provider.connection.confirmTransaction({
      blockhash: latestBuyerBlockhash.blockhash,
      lastValidBlockHeight: latestBuyerBlockhash.lastValidBlockHeight,
      signature: airdropBuyerSig,
    });

    nftMint = await createMint(
      provider.connection,
      seller,
      seller.publicKey,
      seller.publicKey,
      0
    );

    sellerTokenAccount = await createAccount(
      provider.connection,
      seller,
      nftMint,
      seller.publicKey
    );

    buyerTokenAccount = await createAccount(
      provider.connection,
      buyer,
      nftMint,
      buyer.publicKey
    );

    await mintTo(
      provider.connection,
      seller,
      nftMint,
      sellerTokenAccount,
      seller,
      sellerNftTokenAmount
    );

    const _sellerTokenAccount = await getAccount(
      provider.connection,
      sellerTokenAccount
    );

    const _buyerTokenAccount = await getAccount(
      provider.connection,
      buyerTokenAccount
    );

    assert.ok(Number(_sellerTokenAccount.amount) == sellerNftTokenAmount);
    assert.ok(_sellerTokenAccount.owner.equals(seller.publicKey));
    assert.ok(_sellerTokenAccount.mint.equals(nftMint));


    assert.ok(Number(_buyerTokenAccount.amount) == 0);
    assert.ok(_buyerTokenAccount.owner.equals(buyer.publicKey));
    assert.ok(_buyerTokenAccount.mint.equals(nftMint));
  });

  it("creates listing", async () => {
    let [_escrowTokenAccount] = await PublicKey.findProgramAddress(
      [
        sellerTokenAccount.toBytes(),
        Buffer.from(anchor.utils.bytes.utf8.encode("escrow-token")),
      ],
      program.programId
    );
    escrowTokenAccount = _escrowTokenAccount;

    let [_escrowInfoAccount, _escrowInfoAccountBump] =
      await PublicKey.findProgramAddress(
        [
          sellerTokenAccount.toBytes(),
        ],
        program.programId
      );
    escrowInfoAccount = _escrowInfoAccount;
    escrowInfoAccountBump = _escrowInfoAccountBump;

    await program.methods
      .createListing(new BN(sellerListingPrice), escrowInfoAccountBump)
      .accounts({
        seller: seller.publicKey,
        sellerToken: sellerTokenAccount,
        nftMint,
        escrowInfo: escrowInfoAccount,
        escrowToken: escrowTokenAccount,
        systemProgram: SystemProgram.programId,
        rent: anchor.web3.SYSVAR_RENT_PUBKEY,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([seller])
      .rpc();
    
    let updatedEscrowInfoAccount = await program.account.escrowInfo.fetch(escrowInfoAccount);

    assert.ok(updatedEscrowInfoAccount.bump == escrowInfoAccountBump)
    assert.ok(updatedEscrowInfoAccount.escrowToken.equals(escrowTokenAccount))
    assert.ok(updatedEscrowInfoAccount.listPrice.toNumber() == sellerListingPrice)
    assert.ok(updatedEscrowInfoAccount.nftMint.equals(nftMint))
    assert.ok(updatedEscrowInfoAccount.sellerKey.equals(seller.publicKey))
    assert.ok(updatedEscrowInfoAccount.sellerToken.equals(sellerTokenAccount))

    const updatedSellerTokenAccount = await getAccount(
      provider.connection,
      sellerTokenAccount
    );

    const updatedEscrowTokenAccount = await getAccount(
      provider.connection,
      escrowTokenAccount
    );

    assert.ok(Number(updatedSellerTokenAccount.amount) == 0)
    assert.ok(Number(updatedEscrowTokenAccount.amount) == 1)
    assert.ok(updatedEscrowTokenAccount.owner.equals(escrowInfoAccount))

  });

  it("purchases listing", async () => {
    const beforeBuyerLamports = await provider.connection.getBalance(buyer.publicKey);
    const beforeSellerLamports = await provider.connection.getBalance(seller.publicKey);

    await program.methods
      .purchaseListing()
      .accounts({
        buyer: buyer.publicKey,
        buyerToken: buyerTokenAccount,
        nftMint,
        seller: seller.publicKey,
        escrowInfo: escrowInfoAccount,
        escrowToken: escrowTokenAccount,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .signers([buyer])
      .rpc();

    const updatedBuyerTokenAccount = await getAccount(
      provider.connection,
      buyerTokenAccount
    );

    const afterBuyerLamports = await provider.connection.getBalance(buyer.publicKey);
    const afterSellerLamports = await provider.connection.getBalance(seller.publicKey);
    assert.ok(Number(updatedBuyerTokenAccount.amount) == 1)
    assert.ok(beforeBuyerLamports - afterBuyerLamports == 1e9)
    assert.ok(afterSellerLamports - beforeSellerLamports > 1e9)
  });
});
