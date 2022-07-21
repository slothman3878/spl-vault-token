import * as anchor from "@project-serum/anchor";
import { web3, Program, SystemProgram, SplToken, Spl, Native, } from "@project-serum/anchor";
import { 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  TOKEN_PROGRAM_ID, 
  createMint, 
  createAssociatedTokenAccount, 
  mintTo,
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
  const program = anchor.workspace.SplVaultToken as Program<SplVaultToken>;

  let [solend_info, solend_info_bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('solend_info', 'utf-8'), accounts.DEVNET_SOL_MINT.toBuffer()],
    program.programId
  );

  let wSolTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, wallet, NATIVE_MINT, wallet.publicKey
  );

  let collateralTokenAccount = await getOrCreateAssociatedTokenAccount(
    connection, wallet, accounts.DEVNET_SOL_CTOKEN, wallet.publicKey
  );

  let token_mint = accounts.DEVNET_SOL_MINT;
    
  let [vault_info, vault_info_bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('vault_info', 'utf-8'),
    token_mint.toBuffer(),],
    program.programId
  );
  let [pool, pool_bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('pool', 'utf-8'),
    vault_info.toBuffer()],
    program.programId
  );
  let [vault_token_mint, vault_token_mint_bump] = await web3.PublicKey.findProgramAddress(
    [Buffer.from('vault_token_mint', 'utf-8'),
    vault_info.toBuffer()],
    program.programId
  );

  const tx = await program.methods.initialize().accounts({
    vaultInfo: vault_info,
    pool: pool,
    mint: token_mint,
    vaultTokenMint: vault_token_mint,
    /// the rest
    vaultCreator: wallet.publicKey,
    systemProgram: web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: web3.SYSVAR_RENT_PUBKEY,
    // associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
  }).signers([]).rpc();
})()