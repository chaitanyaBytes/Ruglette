import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ruglette } from "../target/types/ruglette";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  AnchorUtils,
  InstructionUtils,
  Queue,
  Randomness,
  asV0Tx,
  ON_DEMAND_DEVNET_PID,
} from "@switchboard-xyz/on-demand";

let randomnessAccount: PublicKey;
let randomness: Randomness;
let sbQueue: PublicKey;
let sbProgram: Program;

describe("ruglette", () => {
  before(async () => {
    const { keypair, connection, program: sbProgramInstance } = await AnchorUtils.loadEnv();
    console.log("keypair", keypair.publicKey.toString());
    console.log("connection", connection.rpcEndpoint); // this is the devnet cluster

    const sbQueueInstance = new PublicKey("FfD96yeXs4cxZshoPPSKhSPgVQxLAJUT3gefgh84m1Di");
    const queueAccount = new Queue(sbProgramInstance, sbQueueInstance);
    console.log("Program", sbProgramInstance!.programId.toString());
    console.log("Queue account", queueAccount.pubkey.toString());

    // create randomness account and initialise it
    const rngKp = Keypair.generate();
    const [randomnessInstance, ix] = await Randomness.create(sbProgramInstance, rngKp, sbQueueInstance);
    console.log("\nCreated randomness account..");
    console.log("Randomness account", randomnessInstance.pubkey.toString());
    randomnessAccount = randomnessInstance.pubkey;
    randomness = randomnessInstance;
    sbQueue = sbQueueInstance;
    sbProgram = sbProgramInstance;
  });

  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.getProvider()

  const connection = provider.connection;
  console.log("connection", connection.rpcEndpoint); // this is the local cluster

  const program = anchor.workspace.ruglette as Program<Ruglette>;
  const programId = program.programId;

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
      ...[authority, player].map((a) =>
        SystemProgram.transfer({
          fromPubkey: provider.publicKey,
          toPubkey: a.publicKey,
          lamports: 100 * LAMPORTS_PER_SOL
        })
      ),

      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: houseVault,
        lamports: 100 * LAMPORTS_PER_SOL
      }),
    ]

    await provider.sendAndConfirm(tx).then(log);

    console.log("authority balance: ", await connection.getBalance(authority.publicKey))
    console.log("player balance: ", await connection.getBalance(player.publicKey))
    console.log("house_vault balance: ", await connection.getBalance(houseVault))
  });

  it("Initialize game", async () => {
    await program.methods.initializeGame(
      new anchor.BN(0.01 * LAMPORTS_PER_SOL),
      new anchor.BN(100 * LAMPORTS_PER_SOL),
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
    await program.methods.placeBet(
      [
        {
          betType: {
            straight: {},
          },
          targets: Buffer.from([1]),
          amount: new anchor.BN(1 * LAMPORTS_PER_SOL),
        },
        {
          betType: {
            black: {},
          },
          targets: Buffer.from([]),
          amount: new anchor.BN(1 * LAMPORTS_PER_SOL),
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
  })

  it("Spin the wheel", async () => {
    // In your spin the wheel test
    const commitIx = await randomness.commitIx(sbQueue);
    const commitTx = new Transaction().add(commitIx);
    await provider.sendAndConfirm(commitTx);

    await program.methods.wheelSpin(
      randomnessAccount
    ).accountsPartial({
      player: player.publicKey,
      round,
      randomnessAccountData: randomnessAccountData.publicKey,
      systemProgram: anchor.web3.SystemProgram.programId
    })
      .signers([player])
      .rpc()
      .then(confirm)
      .then(log);
  })

});
