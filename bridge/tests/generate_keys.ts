// import { Keypair } from "@solana/web3.js";
// import { writeFileSync, existsSync, mkdirSync } from "fs";

// const KEYS_DIR = "tests/keys";

// function saveKeypair(keypair: Keypair, filename: string) {
//   if (!existsSync(KEYS_DIR)) {
//     mkdirSync(KEYS_DIR, { recursive: true });
//   }

//   const keypairArray = Array.from(keypair.secretKey);
//   writeFileSync(`${KEYS_DIR}/${filename}`, JSON.stringify(keypairArray));
//   console.log(`✅ ${filename} saved`);
//   console.log(`   Public Key: ${keypair.publicKey.toString()}`);
// }

// const admin1 = Keypair.generate();
// const admin2 = Keypair.generate();
// const admin3 = Keypair.generate();

// saveKeypair(admin1, "admin1.json");
// saveKeypair(admin2, "admin2.json");
// saveKeypair(admin3, "admin3.json");

// console.log("\n🎉 Admin keypairs generated successfully!");
// console.log("\nAdmin addresses:");
// console.log("admin1:", admin1.publicKey.toString());
// console.log("admin2:", admin2.publicKey.toString());
// console.log("admin3:", admin3.publicKey.toString());
