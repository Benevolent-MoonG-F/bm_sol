import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { Sol1 } from "../target/types/sol_1";

describe("sol_1", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Sol1 as Program<Sol1>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
