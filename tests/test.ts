import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Test } from "../target/types/test";

describe("test", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.test as Program<Test>;

  it("Is initialized!", async () => {
    // Add your test here.
    let provider = anchor.getProvider();
    const [counterPDA] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("counter-acc")],program.programId);
    console.log(counterPDA.toString())
    const tx = await program.methods.initialize().accounts({
      counter: counterPDA,
      authority:provider.wallet.publicKey,
      systemProgram:anchor.web3.SystemProgram.programId
    }).rpc();
    console.log("Your transaction signature", tx);

    let counterData = await program.account.counterAcc.fetch(counterPDA);
    console.log("init value = ",counterData.count.toString())
    const tx1 = await program.methods.increase(new anchor.BN(5)).accounts({
      counter:counterPDA,
    }).rpc();
    let counterData1 = await program.account.counterAcc.fetch(counterPDA);
    console.log("init value = ",counterData1.count.toString())

    const tx2 = await program.methods.decrease(new anchor.BN(5)).accounts({
      counter:counterPDA,
      amount:10
    }).rpc();
    let counterData2 = await program.account.counterAcc.fetch(counterPDA);
    console.log("init value = ",counterData2.count.toString())

  });
});
