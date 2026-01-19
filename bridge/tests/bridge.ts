import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BN } from "bn.js";
import {
  TOKEN_2022_PROGRAM_ID,
  type TOKEN_PROGRAM_ID,
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

console.log("=== FILE LOADED ===");

describe("bridge", async () => {
  console.log("=== INSIDE DESCRIBE ===");
  
  const TOKEN_PROGRAM: typeof TOKEN_2022_PROGRAM_ID | typeof TOKEN_PROGRAM_ID =
    TOKEN_2022_PROGRAM_ID;
  
  console.log("=== TOKEN PROGRAM SET ===");
  
  const provider = anchor.AnchorProvider.env();
  console.log("=== PROVIDER CREATED ===");
  
  anchor.setProvider(provider);

  const user = (provider.wallet as anchor.Wallet).payer;
  const payer = user;
  const connection = provider.connection;

  const program = anchor.workspace.Bridge as Program<Bridge>;

  const accounts: Record<string, PublicKey> = {
    tokenProgram: TOKEN_PROGRAM,
    systemProgram: SystemProgram.programId,
  };
  console.log(" 11111111")
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

    // Найти order PDA с текущим counter
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
    console.log("token1 length:", token1.length);  // Должно быть 40
    console.log("receiver length:", receiver.length);  // Должно быть 40
    console.log("Creating order...");
    console.log("Accounts:", {
      user: accounts.user.toString(),
      orderId: accounts.orderId.toString(),
      order: accounts.order.toString(),
      tokenMint: accounts.tokenMintA.toString(),
      makerTokenAccount: accounts.makerTokenAccount.toString(),
      vault: accounts.vaultTokenAccount.toString(),
      vaultAuthority: accounts.vaultAuthority.toString(),
    });

    const transactionSignature = await program.methods
  .orderForTransfer(
    token1,
    receiver,
    tokenAOfferedAmount,
    tokenBWantedAmount
  )
  .accounts({
     user: alice.publicKey,
    orderId: accounts.orderId,              // ✅ Добавлено
    order: accounts.order,                   // ✅ Добавлено
    token0Mint: tokenMintA.publicKey,        // Было token0Mint, вы написали token_0_mint
    makerTokenAccount: accounts.makerTokenAccount,
    vaultTokenAccount: accounts.vaultTokenAccount,  // ✅ Добавлено
    vaultAuthority: accounts.vaultAuthority,        // ✅ Добавлено
    tokenProgram: TOKEN_PROGRAM,
    systemProgram: SystemProgram.programId, 
  })
  .signers([alice])
  .rpc();
  console.log(transactionSignature,"trantransactionSignature")
    await confirmTransaction(connection, transactionSignature);
    console.log("Order created:", transactionSignature);

    // Проверяем что vault содержит токены
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