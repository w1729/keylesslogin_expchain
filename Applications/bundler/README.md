# Bundler

This repository contains a modified version of the Ethereum Bundler, originally forked from [eth-infinitism](https://github.com/eth-infinitism). The bundler has been customized to meet the specific requirements of this project.

## Running the Bundler Locally

If you are running the bundler locally and connecting to an Anvil testnet instance, you need to enable unsafe mode. To do this, set the following environment variable in your `.env` file:

```bash
BUNDLER_UNSAFE=true
```

This setting allows the bundler to operate in an unprotected mode, which may be necessary for local testing and development. However, it is **not recommended** for production environments due to potential security risks.

## What is a Bundler?

A **Bundler** is a key component in the Account Abstraction (ERC-4337) ecosystem. It is responsible for processing user operations (`UserOps`) and submitting them to the blockchain. The bundler acts as an intermediary, aggregating multiple user operations into a single transaction, thereby optimizing gas efficiency and improving scalability.

### Key Responsibilities of the Bundler

- **Validation of UserOps**: Ensures that submitted operations follow ERC-4337 rules.
- **Gas Fee Management**: Calculates gas costs and prioritizes transactions accordingly.
- **Transaction Submission**: Batches and sends user operations to the Ethereum network.
- **Security Enforcement**: Implements checks to prevent invalid or malicious operations.

## Important Notes

- **For Local Development**: Using `BUNDLER_UNSAFE=true` allows the bundler to bypass certain security measures, making development easier. However, it **should never be used in production** as it can introduce vulnerabilities.
- **Customizations**: This forked version includes modifications specific to the project. If you need additional functionality, ensure to review and test changes properly.

---
