import * as anchor from "@project-serum/anchor";
import { web3 } from '@project-serum/anchor';

export const DEVNET_PROGRAM_ID = new web3.PublicKey("ALend7Ketfx5bxh6ghsCDXAoDrhvEmsXT3cynB6aPLgx");
export const DEVNET_LENDING_MARKET = new web3.PublicKey("GvjoVKNjBvQcFaSKUW1gTE7DxhSpjHbE69umVR5nPuQp");
export const DEVNET_LENDING_MARKET_AUTH = new web3.PublicKey("EhJ4fwaXUp7aiwvZThSUaGWCaBQAJe3AEaJJJVCn3UCK");

export const DEVNET_USDC_RESERVE = new web3.PublicKey("FNNkz4RCQezSSS71rW2tvqZH1LCkTzaiG7Nd1LeA5x5y");
export const DEVNET_USDC_CTOKEN = new web3.PublicKey("E2PSSXsXJGdpqhhaV3rYPpuy1inRCQAWxcdykA1DTmYr");
export const DEVNET_USDC_RESERVE_LIQ_SUPPLY = new web3.PublicKey("HixjFJoeD2ggqKgFHQxrcJFjVvE5nXKuUPYNijFg7Kc5");
export const DEVNET_USDC_MINT = new web3.PublicKey("zVzi5VAf4qMEwzv7NXECVx5v2pQ7xnqVVjCXZwS9XzA");

export const DEVNET_SOL_MINT = new web3.PublicKey("So11111111111111111111111111111111111111112");
export const DEVNET_SOL_RESERVE = new web3.PublicKey("5VVLD7BQp8y3bTgyF5ezm1ResyMTR3PhYsT4iHFU8Sxz");
export const DEVNET_SOL_CTOKEN = new web3.PublicKey("FzwZWRMc3GCqjSrcpVX3ueJc6UpcV6iWWb7ZMsTXE3Gf");
export const DEVNET_SOL_RESERVE_LIQ_SUPPLY = new web3.PublicKey("furd3XUtjXZ2gRvSsoUts9A5m8cMJNqdsyR2Rt8vY9s");

export const MAINNET_PROGRAM_ID = new web3.PublicKey("So1endDq2YkqhipRh3WViPa8hdiSpxWy6z3Z6tMCpAo");
export const MAINNET_LENDING_MARKET = new web3.PublicKey("4UpD2fh7xH3VP9QQaXtsS1YY3bxzWhtfpks7FatyKvdY");
export const MAINNET_LENDING_MARKET_AUTH = new web3.PublicKey("DdZR6zRFiUt4S5mg7AV1uKB2z1f1WzcNYCaTEEWPAuby");

export const MAINNET_USDC_RESERVE = new web3.PublicKey("BgxfHJDzm44T7XG68MYKx7YisTjZu73tVovyZSjJMpmw");
export const MAINNET_USDC_CTOKEN = new web3.PublicKey("993dVFL2uXWYeoXuEBFXR4BijeXdTv4s6BzsCjJZuwqk");
export const MAINNET_USDC_RESERVE_LIQ_SUPPLY = new web3.PublicKey("8SheGtsopRUDzdiD6v6BR9a6bqZ9QwywYQY99Fp5meNf");
export const MAINNET_USDC_MINT = new web3.PublicKey("EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v");

export const getAccounts = () => {
  
}