import { readFileSync } from "fs";
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  TOKEN_2022_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
} from "@solana/spl-token";
import { LAMPORTS_PER_SOL, PublicKey, SystemProgram } from "@solana/web3.js";
import { assert } from "chai";
import type { Bridge } from "../target/types/bridge";


import {
  confirmTransaction,
  createAccountsMintsAndTokenAccounts,
  makeKeypairs,
} from "@solana-developers/helpers";
import { token } from "@coral-xyz/anchor/dist/cjs/utils";

console.log("=== FILE LOADED ===");

describe("bridge", async () => {
  console.log("=== INSIDE DESCRIBE ===");
  
  // const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID =
  //   TOKEN_2022_PROGRAM_ID;
    const TOKEN_PROGRAM = TOKEN_PROGRAM_ID; 
  console.log("=== TOKEN PROGRAM SET ===");
  
  const provider = anchor.AnchorProvider.env();
  console.log("=== PROVIDER CREATED ===");
  
  anchor.setProvider(provider);

  const user = (provider.wallet as anchor.Wallet).payer;
  const payer = user;
  const connection = provider.connection;

  //const program = anchor.workspace.Bridge as Program<Bridge>;
  const program = anchor.workspace.Bridge as Program<Bridge>;
  console.log("=== PROGRAM LOADED ===");
  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM,
    systemProgram: SystemProgram.programId,
  };
  console.log("=== PROVIDER erbererberberberb ===");
///// calling initialize before creating orders
  let admin1: anchor.web3.Keypair = anchor.web3.Keypair.generate();
  await provider.connection.requestAirdrop(admin1.publicKey, 10000000000);
  const airdropSig = await provider.connection.requestAirdrop(
      admin1.publicKey,
      2 * anchor.web3.LAMPORTS_PER_SOL
    );
    await provider.connection.confirmTransaction(airdropSig, "confirmed");
  // const balance  = await provider.connection.getBalance(admin1.publicKey);
  // console.log(balance,"admin1 balance")
  const [adminConfigPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("adminconfig")], program.programId);;//receiving admin config account to init
  console.log(adminConfigPDA,"adminConfigPDA before init")
  // const isAdmin = await program.account.adminConfig.fetch(adminConfigPDA);
  // console.log(isAdmin,"isAdmin before init")


  const inittx = await program.methods
    .initialize([admin1.publicKey])
    .accounts({
      authority: admin1.publicKey
    })
    .signers([admin1])
    .rpc();
  console.log("Transaction signature:", inittx);
  
  ///////end of init
  let alice: anchor.web3.Keypair;
  let tokenMintA: anchor.web3.Keypair;
  [alice, tokenMintA] = makeKeypairs(2);
  const tokenAOfferedAmount = new BN("1000000");
  const tokenBWantedAmount = new BN("1000000");

  const SECONDS = 1000;
  const ANCHOR_SLOW_TEST_THRESHOLD = 40 * SECONDS;
  console.log(ANCHOR_SLOW_TEST_THRESHOLD,"ANCHOR_SLOW_TEST_THRESHOLD");
  before(
    "Creates Alice account, token mint, and associated token accounts",
    async () => {
      const usersMintsAndTokenAccounts =
        await createAccountsMintsAndTokenAccounts(
          [[1_000_000_000]],
          1 * LAMPORTS_PER_SOL,
          connection,
          payer
        );

      const users = usersMintsAndTokenAccounts.users;
      alice = users[0];
      
      const mints = usersMintsAndTokenAccounts.mints;
      tokenMintA = mints[0];
      
      const tokenAccounts = usersMintsAndTokenAccounts.tokenAccounts;
      const aliceTokenAccountA = tokenAccounts[0][0];
      
      // Save the accounts for later use
      accounts.user = alice.publicKey;
      accounts.tokenMintA = tokenMintA.publicKey;
      accounts.makerTokenAccount = aliceTokenAccountA;
    }
  );

  it("Puts the tokens Alice offers into the vault when Alice makes an order", async () => {
   
    const [orderIdPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("order_id")],
      program.programId
    );
    
    accounts.orderId = orderIdPda;

    
    let currentCounter = new BN(0);
    try {
      const orderIdAccount = await program.account.orderId.fetch(orderIdPda);
      currentCounter = new BN(orderIdAccount.counter);
    } catch (e) {
      
      currentCounter = new BN(0);
    }
    const [orderPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("order"),
        alice.publicKey.toBuffer(),
        currentCounter.toArrayLike(Buffer, "le", 8),
      ],
      program.programId
    );
    
    accounts.order = orderPda;
    const [vaultPda] = PublicKey.findProgramAddressSync(
      [
        Buffer.from("vault"),
        tokenMintA.publicKey.toBuffer(),
      ],
      program.programId
    );
    accounts.vaultTokenAccount = vaultPda;
    const [vaultAuthorityPda] = PublicKey.findProgramAddressSync(
      [Buffer.from("vault_authority")],
      program.programId
    );
    
    accounts.vaultAuthority = vaultAuthorityPda;
    // Ethereum адреса (20 байт без 0x префикса)
    const token1 = "0xc5c949ffcd5872731A39d9B33812B9a26b275ebd";
    const receiver = "0xc5c949ffcd5872731A39d9B33812B9a26b275ebd";

try {
  const createOrder = await program.methods.orderForTransfer(token1,receiver, tokenAOfferedAmount, tokenBWantedAmount).accounts({
      user: accounts.user,
      orderId: accounts.orderId,
      order: accounts.order,
      tokenMint: accounts.tokenMintA,
      makerTokenAccount: accounts.makerTokenAccount,
      vault: accounts.vaultTokenAccount,
      vaultAuthority: accounts.vaultAuthority,
      tokenProgram: accounts.tokenProgram,
      systemProgram: accounts.systemProgram,
    })
    .signers([alice])
    .rpc({ 
      skipPreflight: false,  // Включить preflight для проверки
      commitment: "confirmed" 
    });

  console.log("✅ Success! Tx:", createOrder);

} catch (error) {
  console.error("❌ Full error object:", JSON.stringify(error, null, 2));
  if (error instanceof anchor.AnchorError) {
    console.error("Anchor Error Code:", error.error.errorCode.code);
    console.error("Anchor Error Name:", error.error.errorCode.number);
    console.error("Anchor Error Message:", error.error.errorMessage);
  }
  if (error.logs) {
    console.error("\n📋 Program Logs:");
    error.logs.forEach(log => console.error(log));
  }
  if (error.simulationResponse) {
    console.error("\n🔍 Simulation Error:", error.simulationResponse.err);
  }
}
///////////////////////////
    const vaultBalanceResponse = await connection.getTokenAccountBalance(
      vaultPda
    );
    const vaultBalance = new BN(vaultBalanceResponse.value.amount);
    
    console.log("Vault balance:", vaultBalance.toString());
    assert(
      vaultBalance.eq(tokenAOfferedAmount),
      `Expected vault balance ${tokenAOfferedAmount.toString()}, got ${vaultBalance.toString()}`
    );

    // Проверяем что Order аккаунт содержит правильные данные
    const orderAccount = await program.account.order.fetch(orderPda);

    console.log("Order account data:", {
      id: orderAccount.id.toString(),
      maker: orderAccount.maker.toString(),
      token0: orderAccount.token0.toString(),
      token1: orderAccount.token1,
      token0amount: orderAccount.token0amount.toString(),
      token1amount: orderAccount.token1amount.toString(),
      status: orderAccount.status,
    });

    assert(
      orderAccount.maker.equals(alice.publicKey),
      "Order maker should be Alice"
    );
    assert(
      orderAccount.token0.equals(tokenMintA.publicKey),
      "Token0 should match tokenMintA"
    );
    assert(
      orderAccount.token1 === token1,
      "Token1 should match the Ethereum address"
    );
    assert(
      orderAccount.token0amount.eq(tokenAOfferedAmount),
      "Token0 amount should match offered amount"
    );
    assert(
      orderAccount.token1amount.eq(tokenBWantedAmount),
      "Token1 amount should match wanted amount"
    );
    assert(
      orderAccount.status.created !== undefined,
      "Order status should be CREATED"
    );
    assert(
      orderAccount.id.eq(currentCounter),
      "Order ID should match counter"
    );

    console.log("✅ All checks passed!");
  }).slow(ANCHOR_SLOW_TEST_THRESHOLD);

  // it("Cancels the order and returns tokens to Alice", async () => {
  //   // Получаем текущий order
  //   const orderIdAccount = await program.account.orderId.fetch(accounts.orderId);
  //   const orderId = new BN(orderIdAccount.counter).sub(new BN(1)); // Последний созданный order

  //   const [orderPda] = PublicKey.findProgramAddressSync(
  //     [
  //       Buffer.from("order"),
  //       alice.publicKey.toBuffer(),
  //       orderId.toArrayLike(Buffer, "le", 8),
  //     ],
  //     program.programId
  //   );

  //   // Проверяем баланс Alice до отмены
  //   const aliceBalanceBefore = await connection.getTokenAccountBalance(
  //     accounts.makerTokenAccount
  //   );
    
  //   console.log("Alice balance before cancel:", aliceBalanceBefore.value.amount);

  //   const transactionSignature = await program.methods
  //     .orderForCancel()
  //     .accounts({
  //       user: accounts.user,
  //       order: orderPda,
  //       token0Mint: accounts.tokenMintA,
  //       makerTokenAccount: accounts.makerTokenAccount,
  //       vaultTokenAccount: accounts.vaultTokenAccount,
  //       vaultAuthority: accounts.vaultAuthority,
  //       tokenProgram: accounts.tokenProgram,
  //       systemProgram: accounts.systemProgram,
  //     })
  //     .signers([alice])
  //     .rpc();

  //   await confirmTransaction(connection, transactionSignature);
  //   console.log("Order cancelled:", transactionSignature);

  //   // Проверяем что токены вернулись Alice
  //   const aliceBalanceAfter = await connection.getTokenAccountBalance(
  //     accounts.makerTokenAccount
  //   );
    
  //   console.log("Alice balance after cancel:", aliceBalanceAfter.value.amount);
    
  //   const balanceIncrease = new BN(aliceBalanceAfter.value.amount).sub(
  //     new BN(aliceBalanceBefore.value.amount)
  //   );
    
  //   assert(
  //     balanceIncrease.eq(tokenAOfferedAmount),
  //     `Expected balance increase of ${tokenAOfferedAmount.toString()}, got ${balanceIncrease.toString()}`
  //   );

  //   // Проверяем статус ордера
  //   const orderAccount = await program.account.order.fetch(orderPda);
  //   assert(
  //     orderAccount.status.cancelled !== undefined,
  //     "Order status should be CANCELLED"
  //   );

  //   console.log("✅ Cancel test passed!");
  // }).slow(ANCHOR_SLOW_TEST_THRESHOLD);
});