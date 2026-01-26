import { readFileSync } from "fs";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMint,
  createAssociatedTokenAccount,
  mintTo,
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram, Keypair } from "@solana/web3.js";
import { assert } from "chai";
import type { Bridge } from "../target/types/bridge";

console.log("=== FILE LOADED ===");

describe("bridge", async () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const user = (provider.wallet as anchor.Wallet).payer;
  const payer = user;
  const connection = provider.connection;

  const program = anchor.workspace.Bridge as Program<Bridge>;

  let admin1: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  await provider.connection.requestAirdrop(admin1.publicKey, 10000000000);
  const airdropSig = await provider.connection.requestAirdrop(
    admin1.publicKey,
    2 * anchor.web3.LAMPORTS_PER_SOL
  );
  await provider.connection.confirmTransaction(airdropSig, "confirmed");
  const [adminConfigPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("adminconfig")],
    program.programId
  );

  const [orderIdPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("order_id")],
    program.programId
  );
  try {
    const init = await program.methods
      .initialize([admin1.publicKey])
      .accounts({ authority: admin1.publicKey })
      .signers([admin1])
      .rpc();
    console.log("Program initialized:", init);
  } catch (error) {
    console.error("Init error:", error);
  }


  const alice = Keypair.generate();
  await provider.connection.requestAirdrop(alice.publicKey, 2 * LAMPORTS_PER_SOL);


  const tokenMintA = await createMint(
    connection,
    payer,
    alice.publicKey, // mint authority
    alice.publicKey, // freeze authority
    6, // decimals
    undefined,
    undefined,
    TOKEN_PROGRAM_ID
  );


  const aliceTokenAccountA = await createAssociatedTokenAccount(
    connection,
    payer,
    tokenMintA,
    alice.publicKey,
    undefined,
    TOKEN_PROGRAM_ID
  );

  await mintTo(
    connection,
    payer,
    tokenMintA,
    aliceTokenAccountA,
    alice,
    1_000_000_000,
    undefined,
    undefined,
    TOKEN_PROGRAM_ID
  );

  const tokenAOfferedAmount = new BN("1000000");
  const tokenBWantedAmount = new BN("1000000");
  const token1 = "0xc5c949ffcd5872731A39d9B33812B9a26b275ebd";
  const receiver = "0xc5c949ffcd5872731A39d9B33812B9a26b275ebd";

  let currentCounter = new BN(0);
  try {
    const orderIdAccount = await program.account.orderId.fetch(orderIdPDA);
    currentCounter = new BN(orderIdAccount.counter);
  } catch (e) {
    currentCounter = new BN(0);
  }

  const [orderPDA] = PublicKey.findProgramAddressSync(
    [
      Buffer.from("order"),
      alice.publicKey.toBuffer(),
      currentCounter.toArrayLike(Buffer, "le", 8),
    ],
    program.programId
  );

  const [vaultPDA] = PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), tokenMintA.toBuffer()],
    program.programId
  );
  const vaultATA = getAssociatedTokenAddressSync(
    tokenMintA,
    adminConfigPDA,
    true,
    TOKEN_PROGRAM_ID
  );
  try {
    console.log("Creating order...");

    console.log("user: ", alice.publicKey.toString(),);
    console.log("orderId: ", orderIdPDA.toString(),);
    console.log(" order: ", orderPDA.toString(),);
    console.log(' token0Mint:', tokenMintA.toString(),);
    console.log(" makerTokenAccount:", aliceTokenAccountA.toString(),);
    console.log("tokenProgram:", TOKEN_PROGRAM_ID.toString(),);
    console.log(" vaultTokenAccount: ", vaultATA.toString(),);
    console.log("vaultAuthority:", adminConfigPDA.toString(),);
    console.log("systemProgram:", SystemProgram.programId.toString(),);
    console.log("associatedTokenProgram:", ASSOCIATED_TOKEN_PROGRAM_ID.toString())

    const createOrder = await program.methods
      .orderForTransfer(token1, receiver, tokenAOfferedAmount, tokenBWantedAmount)
      .accountsStrict({
        user: alice.publicKey,
        orderId: orderIdPDA,
        order: orderPDA,
        token0Mint: tokenMintA,
        makerTokenAccount: aliceTokenAccountA,
        tokenProgram: TOKEN_PROGRAM_ID,
        vaultTokenAccount: vaultATA,
        vaultAuthority: adminConfigPDA,
        systemProgram: SystemProgram.programId,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([alice])
      .rpc({
        skipPreflight: false,
        commitment: "confirmed",
      });

    console.log("✅ Success! Tx:", createOrder);
  } catch (error: any) {
    console.error("\n" + "=".repeat(60));
    console.error("❌ TRANSACTION FAILED");
    console.error("=".repeat(60));

    console.error("Error:", error.message);

    if (error.logs) {
      console.error("\nProgram Logs:");
      error.logs.forEach((log: string, i: number) => console.error(`  [${i}] ${log}`));
    }
    console.error("\n" + "=".repeat(60) + "\n");
    throw error;
  }
  const checkOrder = await PublicKey.findProgramAddressSync(
    [Buffer.from("order"), alice.publicKey.toBuffer(),],
    program.programId
  );
  const orderAccount = await program.account.order.fetch(orderPDA);
  assert.equal(orderAccount.token1.toString(), token1.toString())
});
