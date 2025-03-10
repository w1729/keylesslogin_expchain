import "dotenv/config";
import { ethers } from "ethers";
import HTTPServer from "moleculer-web";
import { ServiceBroker } from "moleculer";
import snarkjs from "snarkjs";
import fs from "fs";

const signer = new ethers.Wallet(process.env.LOGIN_SERVICE_PK);
const broker = new ServiceBroker();

broker.createService({
  name: "gateway",
  mixins: [HTTPServer],

  settings: {
    port: process.env.LOGIN_SERVICE_PORT ?? 4340,
    routes: [
      {
        aliases: {
          "POST /login": "login.test",
          "POST /generate-proof": "snark.generateProof",
          "POST /verify-snark": "snark.verifyProof",
        },
        cors: {
          origin: "*",
        },
        bodyParsers: {
          json: true,
          urlencoded: { extended: true },
        },
      },
    ],
  },
});

enum SignatureTypes {
  NONE,
  WEBAUTHN_UNPACKED,
  LOGIN_SERVICE,
  WEBAUTHN_UNPACKED_WITH_LOGIN_SERVICE,
}

// Login service
broker.createService({
  name: "login",
  actions: {
    async test(ctx) {
      const { login, credId, pubKeyCoordinates } = ctx.params;

      const message = ethers.utils.keccak256(
        ethers.utils.defaultAbiCoder.encode(
          ["bytes1", "string", "bytes", "uint256[2]"],
          [SignatureTypes.LOGIN_SERVICE, login, credId, pubKeyCoordinates]
        )
      );
      const signature = await signer.signMessage(Buffer.from(message.slice(2), "hex"));
      const payload = ethers.utils.defaultAbiCoder.encode(
        ["bytes1", "string", "bytes", "uint256[2]", "bytes"],
        [SignatureTypes.LOGIN_SERVICE, login, credId, pubKeyCoordinates, signature]
      );
      return payload;
    },
  },
});

// SNARK proof generation & verification service
broker.createService({
  name: "snark",
  actions: {
    async generateProof(ctx) {
      const { input } = ctx.params;
      try {
        const { proof, publicSignals } = await snarkjs.groth16.fullProve(input, "assets/main.wasm", "assets/main_fast.zkey");
        return { proof, publicSignals };
      } catch (error) {
        return { error: error.message };
      }
    },

    async verifyProof(ctx) {
      const { proof, publicSignals } = ctx.params;
      try {
        const vkey = JSON.parse(fs.readFileSync("assets/verification_key.json", "utf-8"));
        const isValid = await snarkjs.groth16.verify(vkey, publicSignals, proof);
        
        if (!isValid) {
          return { verified: false };
        }

        // Generate Solidity input file format
        const solidityInput = {
          proof: {
            a: proof.pi_a.slice(0, 2),
            b: proof.pi_b.slice(0, 2).map(arr => [arr[0], arr[1]]),
            c: proof.pi_c.slice(0, 2)
          },
          publicSignals
        };

        return { verified: true, solidityInput };
      } catch (error) {
        return { verified: false, error: error.message };
      }
    },
  },
});

broker.start();
