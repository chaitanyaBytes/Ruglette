import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ruglette } from "../target/types/ruglette";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  AnchorUtils,
  Queue,
  Randomness,
  asV0Tx,
} from "@switchboard-xyz/on-demand";

let randomnessAccount: PublicKey;
let randomness: Randomness;
let sbQueue: PublicKey;
let sbProgram: Program;
let sbIx: TransactionInstruction
let providerKeypair: Keypair
let rngKeypair: Keypair

describe("ruglette", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider()

  const connection = provider.connection;
  console.log("connection", connection.rpcEndpoint); // this is the local cluster

  const program = anchor.workspace.ruglette as Program<Ruglette>;
  const programId = program.programId;

  before(async () => {
    const { keypair, connection, program: sbProgramInstance } = await AnchorUtils.loadEnv();
    console.log("keypair", keypair.publicKey.toString());
    console.log("connection", connection.rpcEndpoint); // this is the devnet cluster

    const sbQueueInstance = new PublicKey("EYiAmGSdsQTuCw413V5BzaruWuCCSDgTPtBGvLkXHbe7");
    const [state] = PublicKey.findProgramAddressSync([Buffer.from('STATE')], sbProgramInstance.programId);
    console.log("state: ", state)
    const queueAccount = new Queue(sbProgramInstance, sbQueueInstance);
    console.log("Program", sbProgramInstance!.programId.toString());
    console.log("Queue account", queueAccount.pubkey.toString());

    // create randomness account and initialise it
    const rngKp = Keypair.generate();
    const [randomnessInstance, ix] = await Randomness.create(sbProgramInstance, rngKp, sbQueueInstance);
    console.log("\nCreated randomness account..");
    console.log("Randomness account", randomnessInstance.pubkey.toString());
    providerKeypair = keypair;
    randomnessAccount = randomnessInstance.pubkey;
    sbIx = ix;
    rngKeypair = rngKp;
    randomness = randomnessInstance;
    sbQueue = sbQueueInstance;
    sbProgram = sbProgramInstance;
  });

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();
    await connection.confirmTransaction({
      signature,
      ...block
    });
    return signature
  }

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );
    return signature;
  }

  const [authority, player, randomnessAccountData] = Array.from({ length: 3 }, () => Keypair.generate());

  const game = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("game"), authority.publicKey.toBuffer()],
    programId
  )[0];

  const houseVault = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("house_vault"), game.toBuffer()],
    programId
  )[0];

  // For a round, you need player's pubkey and start time
  const startTime = Math.floor(Date.now() / 1000); // Current time in seconds
  const round = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("round"),
      player.publicKey.toBuffer(),
      new anchor.BN(startTime).toArrayLike(Buffer, "le", 8)
    ],
    programId
  )[0];

  const bets = anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("bet"),
      player.publicKey.toBuffer(),
      round.toBuffer()
    ],
    programId
  )[0];

  it("Airdrop", async () => {
    let tx = new anchor.web3.Transaction();

    // airdrop a few sol to authority and player
    tx.instructions = [
      ...[authority, player, randomnessAccountData].map((a) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: a.publicKey,
          lamports: 0.2 * LAMPORTS_PER_SOL
        })
      ),

      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: houseVault,
        lamports: 1 * LAMPORTS_PER_SOL
      }),
    ]

    await provider.sendAndConfirm(tx).then(log);

    console.log("authority balance: ", await connection.getBalance(authority.publicKey))
    console.log("player balance: ", await connection.getBalance(player.publicKey))
    console.log("house_vault balance: ", await connection.getBalance(houseVault))
  });

  it("Initialize game", async () => {
    await program.methods.initializeGame(
      new anchor.BN(0.001 * LAMPORTS_PER_SOL),
      new anchor.BN(1 * LAMPORTS_PER_SOL),
      270,
      false
    ).accountsPartial({
      authority: authority.publicKey,
      game,
      houseVault,
      systemProgram: anchor.web3.SystemProgram.programId
    })
      .signers([authority])
      .rpc()
      .then(confirm)
      .then(log);
  })

  it("Initialize round", async () => {
    await program.methods.initializeRound(
      new anchor.BN(startTime),
    ).accountsPartial({
      player: player.publicKey,
      authority: authority.publicKey,
      round,
      game,
      systemProgram: anchor.web3.SystemProgram.programId
    })
      .signers([player])
      .rpc()
      .then(confirm)
      .then(log);
  })

  it("Place bets", async () => {
    try {

      await program.methods.placeBet(
        [
          {
            betType: {
              straight: {},
            },
            targets: Buffer.from([1]),
            amount: new anchor.BN(0.005 * LAMPORTS_PER_SOL),
          },
          {
            betType: {
              black: {},
            },
            targets: Buffer.from([]),
            amount: new anchor.BN(0.005 * LAMPORTS_PER_SOL),
          }
        ]
      ).accountsPartial({
        player: player.publicKey,
        authority: authority.publicKey,
        round,
        game,
        bets,
        houseVault,
        systemProgram: anchor.web3.SystemProgram.programId
      })
        .signers([player])
        .rpc()
        .then(confirm)
        .then(log);
    } catch (e) {
      console.log("place bets error: ", e)
    }
  })

  it("create a randomness account", async () => {
    console.log("rngkp: ", rngKeypair.publicKey.toString())
    const createRandomnessTx = await asV0Tx({
      connection: provider.connection,
      ixs: [sbIx],
      payer: providerKeypair.publicKey,
      signers: [providerKeypair, rngKeypair],
      computeUnitPrice: 75_000,
      computeUnitLimitMultiple: 1.3,
    });

    // const sim = await connection.simulateTransaction(createRandomnessTx);
    const sig1 = await connection.sendTransaction(createRandomnessTx, { skipPreflight: true });
    await connection.confirmTransaction(sig1)
    console.log(
      "  Transaction Signature for randomness account creation: ",
      sig1
    );
  })

  it("Spin the wheel", async () => {
    // In your spin the wheel test
    console.log("randomnessAccount", randomnessAccount.toString())
    console.log("randomnessAccountData", randomnessAccountData.publicKey.toString())
    console.log("sbQueue", sbQueue.toString())
    console.log("sbProgram", sbProgram.programId.toString())
    console.log("randomness", randomness.pubkey.toString())

    try {
      const commitIx = await randomness.commitIx(sbQueue);

      await program.methods.wheelSpin(
        randomnessAccount
      ).accountsPartial({
        player: player.publicKey,
        round,
        randomnessAccountData: randomnessAccount,
        systemProgram: anchor.web3.SystemProgram.programId
      })
        .preInstructions([commitIx])
        .signers([player])
        .rpc()
    } catch (e) {
      console.error(`Oops, something went wrong: ${e}`)
    }
  })

  it("Verify the randomness", async () => {
    const maxRetries = 3;
    let retryCount = 0;

    while (retryCount < maxRetries) {
      try {
        const revealIx = await randomness.revealIx();

        await program.methods.verifyRandomness().accountsPartial({
          player: player.publicKey,
          authority: authority.publicKey,
          round,
          randomnessAccountData: randomnessAccount,
          systemProgram: anchor.web3.SystemProgram.programId
        })
          .preInstructions([revealIx])
          .signers([player])
          .rpc()
          .then(confirm)
          .then(log);

        break; // If successful, break the retry loop
      } catch (error) {
        retryCount++;
        if (retryCount === maxRetries) {
          throw error; // If we've exhausted retries, throw the error
        }
        // Wait before retrying
        await new Promise(resolve => setTimeout(resolve, 3000));
      }
    }
  })

  it("Settle bets", async () => {
    await program.methods.settleBets().accountsPartial({
      player: player.publicKey,
      authority: authority.publicKey,
      round,
      game,
      bets,
      houseVault,
      systemProgram: anchor.web3.SystemProgram.programId
    })
      .signers([player])
      .rpc()
      .then(confirm)
      .then(log);
  })
});
