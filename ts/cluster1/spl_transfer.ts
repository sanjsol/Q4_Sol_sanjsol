import { Commitment, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey } from "@solana/web3.js"
import wallet from "../wba-wallet.json"
import { getOrCreateAssociatedTokenAccount, transfer } from "@solana/spl-token";

// We're going to import our keypair from the wallet file
const keypair = Keypair.fromSecretKey(new Uint8Array(wallet));

//Create a Solana devnet connection
const commitment: Commitment = "confirmed";
const connection = new Connection("https://api.devnet.solana.com", commitment);
const token_decimals = 1_000_000n;
// Mint address
const mint = new PublicKey("8e4QKDLsx8u7ydKWE7NKSChXVK9Be1rF2HrTpEQRawCh");

// Recipient address
const to = new PublicKey("3PXGWN8a38bEX1wY943u2JdLtJ71EV4ZschgLfuGLcB1");

(async () => {
    try {
        const ata = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, keypair.publicKey);

        // Get the token account of the toWallet address, and if it does not exist, create it
        const toAta = await getOrCreateAssociatedTokenAccount(connection, keypair, mint, to);

        // Transfer the new token to the "toTokenAccount" we just created
        const txid = await transfer(connection, keypair, ata.address, toAta.address, keypair, BigInt(100) * token_decimals);
        console.log(`Your transfer txid: ${txid}`);
    } catch(e) {
        console.error(`Oops, something went wrong: ${e}`)
    }
})();