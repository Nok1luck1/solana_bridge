import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test } from "../target/types/test";

describe("test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.test as Program<Test>;

  it("Is initialized!", async () => {
    // Add your test here.

    const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter")],program.programId);
    console.log(counterPDA.toString())
    // const tx = await program.methods.initialize().rpc();
    // console.log("Your transaction signature", tx);
    // const tx1 = await program.methods.increase().rpc();
  });
});
