import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { SplVaultToken } from "../target/types/spl_vault_token";

describe("spl-vault-token", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SplVaultToken as Program<SplVaultToken>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
