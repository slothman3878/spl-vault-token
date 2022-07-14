import * as anchor from "@project-serum/anchor";
import { web3, Spl } from '@project-serum/anchor';
import { Program, SplToken, SplTokenCoder } from "@project-serum/anchor";
import { 
  ASSOCIATED_TOKEN_PROGRAM_ID, 
  TOKEN_PROGRAM_ID, 
  createMint, 
  createAssociatedTokenAccount, 
  mintTo,
  getAccount, 
} from "@solana/spl-token";
import { SplVaultToken } from "../target/types/spl_vault_token";
import 'dotenv/config';
import * as WALLET from "../wallet.json";
import { VaultRelayer } from "../target/types/vault_relayer";

const WALLET_PRIVATE_KEY: number[] = JSON.parse(process.env.WALLET_PRIVATE_KEY);

describe("vault", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  const connection = provider.connection;
  const wallet = web3.Keypair.fromSecretKey(Uint8Array.from(WALLET_PRIVATE_KEY));
  anchor.setProvider(provider);

  const program = anchor.workspace.SplVaultToken as Program<SplVaultToken>;
  const relay_program = anchor.workspace.VaultRelayer as Program<VaultRelayer>;
  const token_program: Program<SplToken> = Spl.token(provider);

  let token_mint: web3.PublicKey;
  let token_account: web3.PublicKey;
  let vault_token_account: web3.PublicKey;
  let vault_info, pool, vault_token_mint: web3.PublicKey;
  let vault_info_bump, pool_bump, vault_token_mint_bump: number;

  beforeEach('initialize vault', async()=>{
    token_mint = await createMint(
      connection, wallet, wallet.publicKey, wallet.publicKey, 6
    );
    token_account = await createAssociatedTokenAccount(
      connection, wallet, token_mint, wallet.publicKey,
    );
    await mintTo(
      connection, wallet, token_mint, token_account, wallet.publicKey, 100000000
    );
    [vault_info, vault_info_bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from('vault_info', 'utf-8'),
      token_mint.toBuffer(),],
      program.programId
    );
    [pool, pool_bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from('pool', 'utf-8'),
      vault_info.toBuffer()],
      program.programId
    );
    [vault_token_mint, vault_token_mint_bump] = await web3.PublicKey.findProgramAddress(
      [Buffer.from('vault_token_mint', 'utf-8'),
      vault_info.toBuffer()],
      program.programId
    );
    const authority = provider.wallet;
    // Add your test here.
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

    vault_token_account = await createAssociatedTokenAccount(
      connection, wallet, vault_token_mint, wallet.publicKey
    );
  })

  it('Vault Initialize', async ()=>{
    const authority = provider.wallet;
    let old_balance = 0.000000001 * await provider.connection.getBalance(authority.publicKey);
    const tx_deposit = await program.methods.deposit(new anchor.BN(10e6)).accounts({
      owner: wallet.publicKey,
      tokenAccount: token_account,
      vaultTokenAccount: vault_token_account,

      pool: pool,
      vaultTokenMint: vault_token_mint,
      vaultInfo: vault_info,
      tokenProgram: token_program.programId,
    }).signers([
      wallet,
    ]).rpc();
    let new_balance = 0.000000001 * await provider.connection.getBalance(authority.publicKey);
    console.log('difference is', old_balance - new_balance);

    console.log('token balance:', (await getAccount(
      connection, token_account,
    )).amount)
    console.log('vault token balance:', (await getAccount(
      connection, vault_token_account,
    )).amount)

    old_balance = 0.000000001 * await provider.connection.getBalance(authority.publicKey);
    const tx_withdraw = await program.methods.withdraw(new anchor.BN(8e6)).accounts({
      owner: wallet.publicKey,
      tokenAccount: token_account,
      vaultTokenAccount: vault_token_account,
      ///
      pool: pool,
      vaultTokenMint: vault_token_mint,
      vaultInfo: vault_info,
      tokenProgram: token_program.programId,
    }).signers([
      wallet,
    ]).rpc();
    new_balance = 0.000000001 * await provider.connection.getBalance(authority.publicKey);
    console.log('difference is', old_balance - new_balance);

    console.log('token balance:', (await getAccount(
      connection, token_account,
    )).amount)
    console.log('vault token balance:', (await getAccount(
      connection, vault_token_account,
    )).amount)
  })

  it("CPI test", async()=>{
    const authority = provider.wallet;
    let old_balance = 0.000000001 * await provider.connection.getBalance(authority.publicKey);

    const tx_deposit = await relay_program.methods.deposit(new anchor.BN(10e6)).accounts({
      owner: wallet.publicKey,
      sourceLiquidityAccount: token_account,
      destinationCollateralAccount: vault_token_account,
      tokenProgram: token_program.programId,
    }).remainingAccounts([
      {
        pubkey: vault_token_mint,
        isSigner: false,
        isWritable: true,
      },{
        pubkey: vault_info,
        isSigner: false,
        isWritable: false,
      },{
        pubkey: pool,
        isSigner: false,
        isWritable: true,
      },{
        pubkey: program.programId,
        isSigner: false,
        isWritable: false,
      },
    ]).signers([
      wallet,
    ]).rpc();
    let new_balance = 0.000000001 * await provider.connection.getBalance(authority.publicKey);
    console.log('difference is', old_balance - new_balance);
  })
});