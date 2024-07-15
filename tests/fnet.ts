import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Fnet } from "../target/types/fnet";
import NodeWallet from "@coral-xyz/anchor/dist/cjs/nodewallet";
import { Keypair, PublicKey } from "@solana/web3.js";
import { TOKEN_2022_PROGRAM_ID, createAccount, getAccount, getOrCreateAssociatedTokenAccount, createMint } from "@solana/spl-token";
import { createMintWithTransferFee } from "./create-mint";
import { BN, min } from "bn.js";
import { createMetadata } from "./create-metadata";

describe("fnet", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const owner = provider.wallet as NodeWallet;

  const program = anchor.workspace.Fnet as Program<Fnet>;

  const connection = provider.connection;

  const mint = Keypair.generate();

  let currencyMint: PublicKey;
  let currencyTokenAccount: PublicKey;

  const appState = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("app-state"),
      mint.publicKey.toBuffer()
    ],
    program.programId
  )[0];

  const founderToken = Keypair.generate();

  let sixYearToken: PublicKey;

  const firstRound = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("first-round"),
      appState.toBuffer()
    ],
    program.programId
  )[0];

  const secondRound = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("second-round"),
      appState.toBuffer()
    ],
    program.programId
  )[0];

  const thirdRound = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("third-round"),
      appState.toBuffer()
    ],
    program.programId
  )[0];


  const buyer = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("buyer"),
      owner.publicKey.toBuffer()
    ],
    program.programId
  )[0];

  // const currencyPot = PublicKey.findProgramAddressSync(
  //   [
  //     anchor.utils.bytes.utf8.encode("currency-pot"),
  //     currencyMint.toBuffer()
  //   ],
  //   program.programId
  // )[0];

  const [authority, bump] = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("authority"),
      mint.publicKey.toBuffer()
    ],
    program.programId
  );
  
  it("mint founder", async () => {
    // Add your test here.
    const decimals = 6
    const feeBasisPoints = 0
    const maxFee = BigInt(0)

    

    await createMintWithTransferFee(
      connection,
      owner.payer,
      mint,
      decimals,
      feeBasisPoints,
      maxFee,
      authority
    );
    // await createMetadata(
    //   owner.payer,
    //   mint.publicKey,
    //   "Fnet Token",
    //   "FNET",
    //   "https://ipfs.io/ipfs/QmZ1jXCq7Cm1kGX1K6vWaXZwJ6pqKNoK9VKK6EExxGaLap"
    // )
    
    const tx = await program.methods.mintFounder(decimals, bump).accounts({
      mint: mint.publicKey,
      founderToken: founderToken.publicKey,
      authority,
      appState,
      tokenProgram: TOKEN_2022_PROGRAM_ID
    }).signers([founderToken]).rpc().catch(e => console.log(e));
    console.log("Your transaction signature", tx);
    console.log('founderToken::', founderToken);
    console.log('appState::', appState);
    console.log('authority::', authority);
    const account = await getAccount(
      connection,
      founderToken.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(account.amount.toString());
    console.log("mint => ", mint.publicKey.toBase58());
    currencyMint = await createMint(
      connection,
      owner.payer,
      owner.publicKey,
      undefined,
      9
    );
     console.log("currencyMint::", currencyMint);
    const currencyAccount = await getOrCreateAssociatedTokenAccount(
      connection,
      owner.payer,
      currencyMint,
      owner.publicKey
    );
    currencyTokenAccount = currencyAccount.address;
    console.log('currencyAccount::', currencyAccount)
  });
  it("mint one year", async () => {
    const oneYearToken = Keypair.generate();
    const tx = await program.methods.mintOneYear().accounts({
      mint: mint.publicKey,
      appState,
      oneYearToken: oneYearToken.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
      authority
    }).signers([oneYearToken]).rpc();
    console.log(tx);
    const account = await getAccount(
      connection,
      oneYearToken.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID,
    );
    console.log(Number(account.amount));
  });
  it("mint six year", async () => {
    const sixYearTokenAccount = Keypair.generate();
    sixYearToken = sixYearTokenAccount.publicKey;
    const sixYearState = PublicKey.findProgramAddressSync(
      [
        anchor.utils.bytes.utf8.encode("six-year"),
        mint.publicKey.toBuffer()
      ],
      program.programId
    )[0];
    const tx = await program.methods.mintSixYear().accounts({
      mint: mint.publicKey,
      appState,
      sixYearToken: sixYearToken,
      authority,
      sixYearState,
      tokenProgram: TOKEN_2022_PROGRAM_ID
    }).signers([sixYearTokenAccount]).rpc();
    console.log(tx);
    const account = await getAccount(
      connection,
      sixYearToken,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log(account.amount.toString());
  });
  // unlock six year 
  // it("unlock", async () => {
  //   const tokenAccount = await createAccount(
  //     connection,
  //     owner.payer,
  //     mint.publicKey,
  //     owner.publicKey,
  //     undefined,
  //     undefined,
  //     TOKEN_2022_PROGRAM_ID
  //   );

  //   const sixYearState = PublicKey.findProgramAddressSync(
  //     [
  //       anchor.utils.bytes.utf8.encode("six-year"),
  //       mint.publicKey.toBuffer()
  //     ],
  //     program.programId
  //   )[0];

  //   const tx = await program.methods.unlockSixYear().accounts({
  //     mint: mint.publicKey,
  //     appState,
  //     sixYearToken,
  //     tokenAccount,
  //     sixYearState,
  //     tokenProgram: TOKEN_2022_PROGRAM_ID,
  //   }).rpc();
  //   console.log(tx);
  // });
  it("create first round", async () => {
    console.log("-------------------create first round function is started----------")
    const startTime = Math.floor(Date.now() / 1000);
    const endTime = startTime + 2;
    console.log(startTime, endTime);
    const firstRoundToken = Keypair.generate();
    currencyMint = await createMint(
      connection,
      owner.payer,
      owner.publicKey,
      undefined,
      9
    );
    const currencyPot = PublicKey.findProgramAddressSync(
    [
      anchor.utils.bytes.utf8.encode("currency-pot"),
      currencyMint.toBuffer()
    ],
    program.programId
  )[0];

    console.log('firstRoundToken =>', firstRoundToken)
    const tx = await program.methods.createFirstRound(
      new BN(startTime),
      new BN(endTime),
    ).accounts({
      appState,
      mint: mint.publicKey,
      currencyMint,
      authority,
      firstRound,
      firstRoundToken: firstRoundToken.publicKey,
      tokenProgram: TOKEN_2022_PROGRAM_ID,
    }).signers([firstRoundToken]).rpc();
    const account = await getAccount(
      connection,
      firstRoundToken.publicKey,
      undefined,
      TOKEN_2022_PROGRAM_ID
    );
    console.log('first account round account =>', account)
    console.log(account.amount.toString())
    console.log(tx);
  });
  // it("buy in first round", async () => {
  //   console.log('--------------buy in first round -------------');
  //   const currencyPot = PublicKey.findProgramAddressSync(
  //     [
  //       anchor.utils.bytes.utf8.encode("currency-pot"),
  //       currencyMint.toBuffer()
  //     ],
  //     program.programId
  //   )[0];
  //   const dallar_amount = 100;
  //   const tx = await program.methods.buyInFirstRound(new BN(dallar_amount)).accounts({
  //     firstRound,
  //     appState,
  //     buyer,
  //     mint: mint.publicKey,
  //     currencyMint,
  //     // userCurrencyAccount: currencyAccount.address,
  //     currencyPot
  //   }).rpc();
  //   console.log(tx);
  // })
  // it ("finalize first round", async () => {
  //   const firstRoundToken = "";
  //   const tx = await program.methods.finalizeFirstRound().accounts({
  //     appState,
  //     mint: mint.publicKey,
  //     authority,
  //     firstRoundToken,
  //     firstRound
  //   }).rpc();
  //   console.log(tx);
  // })
});
