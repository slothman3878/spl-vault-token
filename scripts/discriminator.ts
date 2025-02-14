import * as anchor from "@project-serum/anchor";
import { web3, Program, SystemProgram, SplToken, Spl, Native, } from "@project-serum/anchor";
import {
  Account,
  getAccount,
  NATIVE_MINT,
  getOrCreateAssociatedTokenAccount,
} from "@solana/spl-token";
import 'dotenv/config';
import * as accounts from './accounts';
import { LAMPORTS_PER_SOL } from "@solana/web3.js";
import { SplVaultToken } from "../target/types/spl_vault_token";
import { VaultWrapper } from "../target/types/vault_wrapper";

const WALLET_PRIVATE_KEY: number[] = JSON.parse(process.env.WALLET_PRIVATE_KEY);

(async ()=>{
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = web3.Keypair.fromSecretKey(Uint8Array.from(WALLET_PRIVATE_KEY));
  anchor.setProvider(provider);
  
  const systemProgram = anchor.Native.system(provider) as Program<SystemProgram>;
  const tokenProgram: Program<SplToken> = Spl.token(provider);
  const program = anchor.workspace.VaultWrapper as Program<VaultWrapper>;

  let [solend_info, solend_info_bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('solend_info', 'utf-8'), accounts.DEVNET_SOL_MINT.toBuffer()],
    program.programId
  );

  let wSolTokenAccount: Account = await getOrCreateAssociatedTokenAccount(
    connection, wallet, NATIVE_MINT, wallet.publicKey
  );

  let collateralTokenAccount: Account = await getOrCreateAssociatedTokenAccount(
    connection, wallet, accounts.DEVNET_SOL_CTOKEN, wallet.publicKey
  );
  
  let depositDiscriminator = (await program.methods.deposit(new anchor.BN(anchor.web3.LAMPORTS_PER_SOL))
    .accounts({
      owner: wallet.publicKey,
      sourceLiquidityAccount: wSolTokenAccount.address,
      destinationCollateralAccount: collateralTokenAccount.address,
      tokenProgram: tokenProgram.programId,
    }).instruction()
  ).data.slice(0, 8);

  let mintToDiscriminator = (await program.methods.mintTo(new anchor.BN(1))
    .accounts({
      owner: wallet.publicKey,
      sourceLiquidityAccount: wSolTokenAccount.address,
      destinationCollateralAccount: collateralTokenAccount.address,
    }).instruction()
  ).data.slice(0,8);

  let redeemDiscriminator = (await program.methods.redeem(new anchor.BN(1))
    .accounts({
      owner: wallet.publicKey,
      sourceCollateralAccount: wSolTokenAccount.address,
      destinationLiquidityAccount: collateralTokenAccount.address,
    }).instruction()
  ).data.slice(0,8);

  let withdrawDiscriminator = (await program.methods.withdraw(new anchor.BN(1))
    .accounts({
      owner: wallet.publicKey,
      sourceCollateralAccount: wSolTokenAccount.address,
      destinationLiquidityAccount: collateralTokenAccount.address,
    }).instruction()
  ).data.slice(0,8);

  console.log('deposit', depositDiscriminator.toJSON().data);
  console.log('mintTo', mintToDiscriminator.toJSON().data);
  console.log('withdraw', withdrawDiscriminator.toJSON().data);
  console.log('redeem', redeemDiscriminator.toJSON().data);
})()