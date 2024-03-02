# Cypher-Zer0x Plasma Rollup Client

Welcome to the official repository of the Cypher-Zer0x Plasma Rollup Client, a high-performance, secure, and decentralized solution designed to scale blockchain technology. Our client leverages the power of Rust programming language, ensuring maximum efficiency and safety.

## Features

- **Multi-Chain Support**: Cypher-Zer0x is designed to operate across multiple blockchains, providing a unified layer 2 scaling solution. Our architecture includes dedicated threads for listening to smart contract events on each supported chain, ensuring real-time responsiveness and interoperability.

- **LMDB for Storage**: Leveraging the Lightning Memory-Mapped Database (LMDB), the client offers high-speed, reliable data storage and retrieval, ensuring your rollup node operates at peak efficiency.

- **Multithreading Architecture**: Our client utilizes a sophisticated multithreading approach to handle various operations concurrently, ensuring high throughput and optimal resource usage. The architecture includes:
  - A thread per chain for listening to smart contract events, allowing seamless multi-chain integration.
  - A dedicated thread for a REST API, enabling users to fetch data from the network and submit transactions with ease.
  - A block producer thread that polls the mempool every 10 seconds to bundle transactions into blocks, ensuring timely and efficient block production.
  - A prover thread that uses the RiscZero stack to generate zero-knowledge proofs for state differences, further enhancing the privacy and security of transactions.

## Getting Started

### Prerequisites

- Rust toolchain (latest stable version recommended)
- LMDB installed on your system

### Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/your-repo/cypher-zer0x-plasma-rollup-client.git
   ````
2. Set up your env variables
3. Set up your network.json
4. Build the client:
  ```bash
  cargo build --release
  ````
5. Run the client:
  ```bash
  cargo run --release
  ```
