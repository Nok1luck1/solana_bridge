import { readFileSync, writeFileSync, existsSync, mkdirSync } from "fs";
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

const KEYS_DIR = "tests/keys";

function saveKeypair(keypair: Keypair, filename: string) {
  if (!existsSync(KEYS_DIR)) {
    mkdirSync(KEYS_DIR, { recursive: true });
  }

  const keypairArray = Array.from(keypair.secretKey);
  writeFileSync(`${KEYS_DIR}/${filename}`, JSON.stringify(keypairArray));
  console.log(`Keypair saved to ${KEYS_DIR}/${filename}`);
}

function loadKeypair(filename: string): Keypair {
  const path = `${KEYS_DIR}/${filename}`;
  if (!existsSync(path)) {
    throw new Error(`Keypair file not found: ${path}`);
  }

  const keypairData = JSON.parse(readFileSync(path, "utf-8"));
  return Keypair.fromSecretKey(new Uint8Array(keypairData));
}

function keypairExists(filename: string): boolean {
  return existsSync(`${KEYS_DIR}/${filename}`);
}

describe("bridge", () => {
  let provider: anchor.AnchorProvider;
  let program: Program<Bridge>;
  let user: Keypair;
  let payer: Keypair;
  let connection: anchor.web3.Connection;

  let admin1: Keypair;
  let admin2: Keypair;
  let admin3: Keypair;
  let admin_config_get: PublicKey;
  let adminConfigPDA: PublicKey;
  let orderIdPDA: PublicKey;
  let orderIdPDAlatest: PublicKey;
  let alice: Keypair;
  let tokenMintA: PublicKey;
  let aliceTokenAccountA: PublicKey;

  let token0amount = new BN("1000000");
  let token1amount = new BN("1000000");
  let token1: string;
  let receiver: string;

  let currentCounter = new BN("1");
  let orderPDA: PublicKey;
  let orderExecutionPDA: PublicKey;
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
      try {
        const adminConfigAccount = await program.account.adminConfig.fetch(
          adminConfigPDA,
        );
        console.log("Program already initialized, loading admins from files");

        // ===== ЗАГРУЗКА СУЩЕСТВУЮЩИХ АДМИНОВ =====
        admin1 = loadKeypair("admin1.json");
        admin2 = loadKeypair("admin2.json");
        admin3 = loadKeypair("admin3.json");

        console.log("Loaded admins:", {
          admin1: admin1.publicKey.toString(),
          admin2: admin2.publicKey.toString(),
          admin3: admin3.publicKey.toString(),
        });

        // Проверь баланс и при необходимости пополни
        const balance1 = await connection.getBalance(admin1.publicKey);
        if (balance1 < LAMPORTS_PER_SOL) {
          const airdropSig = await provider.connection.requestAirdrop(
            admin1.publicKey,
            2 * LAMPORTS_PER_SOL,
          );
          await provider.connection.confirmTransaction(airdropSig, "confirmed");
        }
      } catch (error) {
        console.log("Initializing program with new admins");

        admin1 = Keypair.generate();
        admin2 = Keypair.generate();
        admin3 = Keypair.generate();

        saveKeypair(admin1, "admin1.json");
        saveKeypair(admin2, "admin2.json");
        saveKeypair(admin3, "admin3.json");

        // Airdrop для админов
        const airdropSig = await provider.connection.requestAirdrop(
          admin1.publicKey,
          2 * LAMPORTS_PER_SOL,
        );
        await provider.connection.confirmTransaction(airdropSig, "confirmed");

        const airdropSig2 = await provider.connection.requestAirdrop(
          admin2.publicKey,
          2 * LAMPORTS_PER_SOL,
        );
        await provider.connection.confirmTransaction(airdropSig2, "confirmed");

        const airdropSig3 = await provider.connection.requestAirdrop(
          admin3.publicKey,
          2 * LAMPORTS_PER_SOL,
        );
        await provider.connection.confirmTransaction(airdropSig3, "confirmed");

        console.log("New admins created:", {
          admin1: admin1.publicKey.toString(),
          admin2: admin2.publicKey.toString(),
          admin3: admin3.publicKey.toString(),
        });

        const init = await program.methods
          .initialize([admin1.publicKey, admin2.publicKey, admin3.publicKey])
          .accounts({ authority: admin1.publicKey })
          .signers([admin1])
          .rpc();
        console.log("Program initialized:", init);
        const latestBlockhash = await provider.connection.getLatestBlockhash();
        await provider.connection.confirmTransaction({
          signature: init,
          blockhash: latestBlockhash.blockhash,
          lastValidBlockHeight: latestBlockhash.lastValidBlockHeight,
        });

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

      try {
        const orderIdAccount = await program.account.orderId.fetch(orderIdPDA);
        console.log(orderIdAccount.counter.toString(), "counter acc");
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
          .orderForTransfer(token1, receiver, token0amount, token1amount)
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
        counter: specificOrder.id.toString(),
      });
    });

    it("should cancel order for transfer", async () => {
      const vaults = await connection.getTokenAccountsByOwner(adminConfigPDA, {
        programId: TOKEN_PROGRAM_ID,
      });
      console.log(`Vaults: ${vaults.value.length}`);
      await Promise.all(
        vaults.value.map(async (v) => {
          const balance = await connection.getTokenAccountBalance(v.pubkey);
          const mint = new PublicKey(v.account.data.slice(0, 32));
          console.log(
            `${mint.toString().slice(0, 8)}...: ${balance.value.uiAmount}`,
          );
        }),
      );

      try {
        const cancelOrder = await program.methods
          .cancelExistingOrder()
          .accountsStrict({
            order: orderPDA,
            token0Mint: tokenMintA,
            makerTokenAccount: aliceTokenAccountA,
            vaultTokenAccount: vaultATA,
            admin: adminConfigPDA,
            tokenProgram: TOKEN_PROGRAM_ID,
            adminConfig: admin1.publicKey,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([admin1])
          .rpc({
            skipPreflight: false,
            commitment: "confirmed",
          });
        console.log(cancelOrder);
      } catch (error) {
        console.log(error);
      }
      const orderAccount = await program.account.order.fetch(orderPDA);
      assert.equal(orderAccount.token1.toString(), token1.toString());
    });

    it("should execute order for transfer", async () => {
      [orderIdPDAlatest] = PublicKey.findProgramAddressSync(
        [Buffer.from("order_id")],
        program.programId,
      );
      const orderIdAccount = await program.account.orderId.fetch(
        orderIdPDAlatest,
      );
      console.log(orderIdAccount.counter.toString(), "counter acc");
      currentCounter = orderIdAccount.counter;

      [orderExecutionPDA] = PublicKey.findProgramAddressSync(
        [
          Buffer.from("order_execution"),
          admin1.publicKey.toBuffer(),
          currentCounter.toArrayLike(Buffer, "le", 8),
        ],
        program.programId,
      );

      const timeStart = new BN("123333333");
      [admin_config_get] = PublicKey.findProgramAddressSync(
        [Buffer.from("adminconfig")],
        program.programId,
      );

      console.log("Executing with admin1:", admin1.publicKey.toString());

      const admin_config_array = await program.account.adminConfig.fetch(
        admin_config_get,
      );

      console.log("Admins in config:");
      admin_config_array.admins.forEach((admin, index) => {
        console.log(`Admin ${index}:`, admin.toString());
      });

      // Проверка что admin1 есть в списке
      const isAdmin = admin_config_array.admins.some((a) =>
        a.equals(admin1.publicKey),
      );
      console.log("Is admin1 authorized?", isAdmin);
      assert.isTrue(isAdmin, "admin1 must be in the admin list");

      try {
        const executionOrder = await program.methods
          .orderForExecution(
            alice.publicKey,
            token0amount,
            token1amount,
            token1,
            receiver,
            timeStart,
          )
          .accountsStrict({
            orderId: orderIdPDAlatest,
            orderExecution: orderExecutionPDA,
            token1Mint: tokenMintA,
            receiverTokenAccount: aliceTokenAccountA,
            vaultTokenProgram: vaultATA,
            admin: admin1.publicKey,
            adminConfig: adminConfigPDA,
            tokenProgram: TOKEN_PROGRAM_ID,
            systemProgram: SystemProgram.programId,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          })
          .signers([admin1])
          .rpc({
            skipPreflight: false,
            commitment: "confirmed",
          });
        console.log("Execution successful:", executionOrder);
      } catch (error) {
        console.log("Execution error:", error);
        throw error;
      }
    });
  });
});
// ```

// **Не забудь добавить в `.gitignore`:**
// ```
// tests/keys/
