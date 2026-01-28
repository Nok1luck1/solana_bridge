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
import {
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Keypair,
} from "@solana/web3.js";
import { assert } from "chai";
import type { Bridge } from "../target/types/bridge";

describe("bridge", () => {
  let provider: anchor.AnchorProvider;
  let program: Program<Bridge>;
  let user: Keypair;
  let payer: Keypair;
  let connection: anchor.web3.Connection;

  let admin1: Keypair;
  let adminConfigPDA: PublicKey;
  let orderIdPDA: PublicKey;
  let alice: Keypair;
  let tokenMintA: PublicKey;
  let aliceTokenAccountA: PublicKey;

  let token0amount = new BN("100099");
  let token1amount = new BN("100099");
  let token1: string;
  let receiver: string;

  let currentCounter = new BN("1");
  let orderPDA: PublicKey;
  let vaultPDA: PublicKey;
  let vaultATA: PublicKey;
  async function getAllOrders(filters = []) {
    try {
      const orders = await program.account.order.all(filters);
      return orders;
    } catch (error) {
      console.error("Error fetching orders:", error);
      return [];
    }
  }

  // async function getOrderByPDA(orderPDA) {
  //   try {
  //     const order = await program.account.order.fetch(orderPDA);
  //     return order;
  //   } catch (error) {
  //     console.error(`Error fetching order ${orderPDA.toString()}:`, error);
  //     return null;
  //   }
  // }

  before(async () => {
    provider = anchor.AnchorProvider.env();
    anchor.setProvider(provider);
    program = anchor.workspace.Bridge as Program<Bridge>;
    user = (provider.wallet as anchor.Wallet).payer;
    payer = user;
    connection = provider.connection;

    [adminConfigPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("adminconfig")],
      program.programId,
    );

    [orderIdPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from("order_id")],
      program.programId,
    );
  });

  describe("Initialization", () => {
    it("should initialize program with admin", async () => {
      admin1 = Keypair.generate();
      const airdropSig = await provider.connection.requestAirdrop(
        admin1.publicKey,
        2 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(airdropSig, "confirmed");

      try {
        const adminConfigAccount = await program.account.adminConfig.fetch(
          adminConfigPDA,
        );
        console.log(adminConfigAccount.settet, "settet or not?");
        console.log("Program already initialized, skipping initialization");
      } catch {
        const init = await program.methods
          .initialize([admin1.publicKey])
          .accounts({ authority: admin1.publicKey })
          .signers([admin1])
          .rpc();
        console.log("Program initialized:", init);

        const adminConfigAccount = await program.account.adminConfig.fetch(
          adminConfigPDA,
        );
        assert.include(
          adminConfigAccount.admins.map((a) => a.toString()),
          admin1.publicKey.toString(),
        );
      }
    });
  });

  describe("Test Setup", () => {
    before(async () => {
      alice = Keypair.generate();
      const airdropSig = await provider.connection.requestAirdrop(
        alice.publicKey,
        2 * LAMPORTS_PER_SOL,
      );
      await provider.connection.confirmTransaction(airdropSig, "confirmed");

      tokenMintA = await createMint(
        connection,
        payer,
        alice.publicKey,
        alice.publicKey,
        6,
        undefined,
        undefined,
        TOKEN_PROGRAM_ID,
      );

      aliceTokenAccountA = await createAssociatedTokenAccount(
        connection,
        payer,
        tokenMintA,
        alice.publicKey,
        undefined,
        TOKEN_PROGRAM_ID,
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
        TOKEN_PROGRAM_ID,
      );

      token1 = "0xc5c949ffcd5872731A39d9B33812B9a26b275ebd";
      receiver = "0xc5c949ffcd5872731A39d9B33812B9a26b275ebd";

      [orderIdPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("order_id")],
        program.programId,
      );
      console.log(orderIdPDA);
      try {
        const orderIdAccount = await program.account.orderId.fetch(orderIdPDA);
        console.log(orderIdAccount.counter.toString(), "counter acc")
        currentCounter = orderIdAccount.counter;
      } catch {
        currentCounter = new BN(1);
      }

      [orderPDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("order"),
          alice.publicKey.toBuffer(),
          currentCounter.toArrayLike(Buffer, "le", 8),
        ],
        program.programId,
      );

      [vaultPDA] = PublicKey.findProgramAddressSync(
        [Buffer.from("vault"), tokenMintA.toBuffer()],
        program.programId,
      );

      vaultATA = getAssociatedTokenAddressSync(
        tokenMintA,
        adminConfigPDA,
        true,
        TOKEN_PROGRAM_ID,
      );
    });

    it("should create order for transfer", async () => {
      try {
        const createOrder = await program.methods
          .orderForTransfer(
            token1,
            receiver,
            token0amount,
            token1amount,
          )
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
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([alice])
          .rpc({
            commitment: "confirmed",
            preflightCommitment: "confirmed",
            skipPreflight: false,
          });
      } catch (error) {
        console.log(error);
      }
      const specificOrder = await program.account.order.fetch(orderPDA);
      assert.equal(specificOrder.token1.toString(), token1.toString());
      console.log("Order details:", {
        maker: specificOrder.maker.toString(),
        token1: specificOrder.token1,
        receiver: specificOrder.receiver,
        token0Amount: specificOrder.token0Amount.toString(),
        token1Amount: specificOrder.token1Amount.toString(),
        counter: specificOrder.id,
      });

    });
    it("should cancel order for transfer", async () => {
      const vaults = await connection.getTokenAccountsByOwner(adminConfigPDA, { programId: TOKEN_PROGRAM_ID });
      console.log(`Vaults: ${vaults.value.length}`);
      await Promise.all(vaults.value.map(async (v) => {
        const balance = await connection.getTokenAccountBalance(v.pubkey);
        const mint = new PublicKey(v.account.data.slice(0, 32));
        console.log(`${mint.toString().slice(0, 8)}...: ${balance.value.uiAmount}`);
      }));

      // const getCurrentUserOrder = await ;
      // const cancelOrder = await program.methods
      //   .cancelExistingOrder()
      //   .accountsStrict({
      //     user: alice.publicKey,
      //     order: orderPDA,
      //     token0Mint: tokenMintA,
      //     makerTokenAccount: aliceTokenAccountA,
      //     tokenProgram: TOKEN_PROGRAM_ID,
      //     vaultTokenAccount: vaultATA,
      //     vaultAuthority: adminConfigPDA,
      //     systemProgram: SystemProgram.programId,
      //     associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      //   })
      //   .signers([alice])
      //   .rpc({
      //     skipPreflight: false,
      //     commitment: "confirmed",
      //   });

      // const orderAccount = await program.account.order.fetch(orderPDA);
      // assert.equal(orderAccount.token1.toString(), token1.toString());
    });
  });
});
