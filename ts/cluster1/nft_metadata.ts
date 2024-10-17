import wallet from "../wba-wallet.json"
import { createUmi } from "@metaplex-foundation/umi-bundle-defaults"
import { createGenericFile, createSignerFromKeypair, signerIdentity } from "@metaplex-foundation/umi"
import { irysUploader } from "@metaplex-foundation/umi-uploader-irys"

// Create a devnet connection
const umi = createUmi('https://api.devnet.solana.com');

let keypair = umi.eddsa.createKeypairFromSecretKey(new Uint8Array(wallet));
const signer = createSignerFromKeypair(umi, keypair);

umi.use(irysUploader());
umi.use(signerIdentity(signer));

(async () => {
    try {
        // Follow this JSON structure
        // https://docs.metaplex.com/programs/token-metadata/changelog/v1.0#json-structure

        const image = "https://devnet.irys.xyz/5iSmfi5UpWdn7uE6cqwcpPuKxCgS2Qa8gAVmNXjqRBpP";
        const metadata = {
            name: "Dean Mexican",
            symbol: "DMEX",
            description: "Dean Turbin3 Mexican Version",
            image: image,
            attributes: [
                {
                    trait_type: 'Job',
                    value: 'Turbin3 Mr Olympia'
                },
                {
                    trait_type: 'Nationality',
                    value: 'Mexican as well LOL'
                },
                {
                    trait_type: 'Beard',
                    value: 'Good Moostache, andre is better'
                },
                {
                    trait_type: 'Quality',
                    value: 'Almost Better than Andre'
                }
            ],
            properties: {
                files: [
                    {
                        type: "image/png",
                        uri: image
                    },
                ]
            },
            creators: [keypair.publicKey]
        };
        const myUri = await umi.uploader.uploadJson(metadata);
        console.log("Your metadata URI: ", myUri);
    }
    catch(error) {
        console.log("Oops.. Something went wrong", error);
    }
})();
