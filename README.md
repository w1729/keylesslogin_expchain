# 🚀 Pass Wallet - The Future of Seamless Web3 Access

Pass Wallet is an **Account Abstraction (AA) wallet** designed to provide a seamless, keyless, and gas-free experience for users. With **zkLogin (zero-knowledge login)** and built-in transaction sponsorship, Pass Wallet eliminates the complexities of traditional blockchain interactions, making Web3 accessible to everyone.

---

## 🌟 Features

### 🔐 **Keyless Authentication with zkLogin**

- No private keys required – authenticate securely using your **email**.
- Currently supports **Gmail**, with support for multiple providers coming soon.
- Users can **recover accounts via email** without compromising security.

### ⚡ **One-Click Wallet Creation**

- Create a wallet on the **EXP Chain** in just **two clicks**.
- Enter a username and verify your email – Pass Wallet does the rest.

### 🔑 **Passkey-Based Transaction Signing**

- A passkey is generated and stored securely.
- Users can link a new passkey using **zkProof of email** for seamless account recovery.

### 🏦 **Built-In Paymaster & Bundler**

- **Gas sponsorship** for all transactions – no need to hold native tokens.
- A **custom bundler** ensures fast and efficient transaction processing.

### 🌉 **Multi-Platform Integration**

- Access **swaps, bridges, and dApps** directly from **Pass Wallet**.
- A single wallet interface for all your blockchain interactions.

---

## 🛠️ How It Works

1️⃣ **User Onboarding**: Enter a username and verify your email to create a wallet. <br>
2️⃣ **Wallet Deployment**: A smart contract wallet is automatically deployed. <br>
3️⃣ **Signer Addition**: zkProof authorizes the user as a signer. <br>
4️⃣ **Gas Sponsorship**: Transactions are processed seamlessly via the paymaster. <br>
5️⃣ **Access Web3 Services**: Use swaps, bridges, and more—all from **Pass Wallet**. <br>

---

## Getting Started

## 1. Contract deployment.<br>

To deploy the contract, follow these steps:

### **1. Start Anvil and Fork the EXP Chain**

Run the following command to start Anvil and fork the EXP Chain:

```bash
anvil --fork-url https://rpc0-testnet.expchain.ai
```

### **2. Navigate to the Contracts Directory**

Move into the directory where your smart contracts are stored:

```bash
cd contracts
```

### **3. Deploy the Contracts Using Forge**

Execute the deployment script with Forge:

```bash
forge script script/DeployAllAnvil.s.sol \
  --private-key <private-key> \
  --broadcast -vvv \
  --rpc-url http://localhost:8545
```

This will deploy all contracts to the forked EXP Chain running on Anvil. There is seperate readme is availiable for eac folder.

## **2. Docker Container Deployment**

1. **Navigate to the Application Directory**

   ```bash
   cd Application
   ```

2. **Enable Host Networking**  
   Ensure that host networking is enabled in your Docker settings.

3. **Run Docker in Detached Mode**  
   Execute the following command to start the container:
   ```bash
   docker-compose up -d
   ```
   This will run the Docker containers in the background (detached mode).

## 📌 Roadmap

- ✅ Gmail login support
- ⏳ Integration with multiple identity providers
- ⏳ Expansion to support more blockchain networks
- ⏳ Enhanced security features & account recovery options

---

## Contract Address Exp chain

Zk verifier: 0x2ae470412F8b53E263C8F967E9679e4830a2643A
