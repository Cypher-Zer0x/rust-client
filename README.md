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
- A Bonsai API key, you can request access [here](https://docs.google.com/forms/d/e/1FAIpQLSf9mu18V65862GS4PLYd7tFTEKrl90J5GTyzw_d14ASxrruFQ/viewform)

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

## Available API routes :  
### Metrics Endpoints

#### /metrics
- *Method:* GET
- *Description:* Fetches blockchain metrics.
- *Usage:* Used to get an overview of the blockchain's current metrics.

### Transaction (TX) Endpoints

#### /mempool
- *Method:* GET
- *Description:* Retrieves data about the current state of the mempool.
- *Usage:* Useful for understanding the pending transactions waiting to be confirmed.

#### /utxo/set
- *Method:* GET
- *Description:* Fetches the set of UTXOs.
- *Usage:* Enables querying of unspent transaction outputs, essential for transaction processing and wallet balance calculations.

#### /utxo/hash/:hash
- *Method:* GET
- *Description:* Retrieves UTXO data by its hash.
- *Parameters:*
  - hash: The hash of the UTXO.
- *Usage:* Used for looking up specific UTXO details by hash.

#### /transaction/all
- *Method:* GET
- *Description:* Fetches data for all transactions.
- *Note:* Consider adding a range mechanism to improve efficiency.
- *Usage:* Allows retrieval of comprehensive transaction data, which may become inefficient without a range mechanism.

#### /transaction/hash/:tx_hash
- *Method:* GET
- *Description:* Retrieves transaction data by its hash.
- *Parameters:*
  - tx_hash: The hash of the transaction.
- *Usage:* Useful for querying specific transactions directly.

#### /transaction/number
- *Method:* GET
- *Description:* Fetches the number of transactions.
- *Usage:* Provides a count of all transactions processed.

#### /transaction/latest-ten
- *Method:* GET
- *Description:* Retrieves the latest ten transactions.
- *Usage:* Offers a quick overview of the most recent transactions.

### Block Endpoints

#### /block/all
- *Method:* GET
- *Description:* Fetches data for all blocks.
- *Note:* Consider adding a range mechanism to improve efficiency.
- *Usage:* Enables retrieval of all block data, which may become slow and inefficient without a range mechanism.

#### /block/hash/:block_hash
- *Method:* GET
- *Description:* Retrieves block data by its hash.
- *Parameters:*
  - block_hash: The hash of the block.
- *Usage:* Allows for querying specific blocks directly.

#### /block/number/:block_number
- *Method:* GET
- *Description:* Fetches block data by block number.
- *Parameters:*
  - block_number: The number of the block.
- *Usage:* Useful for retrieving blocks based on their height in the blockchain.

#### /block/range/:rangeData
- *Method:* GET
- *Description:* Retrieves a range of blocks.
- *Parameters:*
  - rangeData: Specifies the range of blocks to retrieve.
- *Usage:* Facilitates the retrieval of blocks within a specific range.

#### /block/latest
- *Method:* GET
- *Description:* Fetches the latest block data.
- *Usage:* Provides data on the most recently mined or produced block.

#### /block/latest-ten
- *Method:* GET
- *Description:* Retrieves the latest ten blocks.
- *Usage:* Offers a snapshot of the ten most recent blocks.

#### /block/total-number
- *Method:* GET
- *Description:* Fetches the total number of blocks.
- *Usage:* Gives the total count of blocks in the blockchain.

### State Endpoints

#### /state/current
- *Method:* GET
- *Description:* Fetches the last proven state of the network.
- *Usage:* Gives the last proven state as a merkle root of block hash.

#### /state/block/current
- *Method:* GET
- *Description:* Fetches the last proven block number of the network.
- *Usage:* Gives the last proven block number.

### User Operation Endpoints

#### /ringct
- *Method:* POST
- *Description:* Handles RingCT (Ring Confidential Transactions) operations.
- *Usage:* Enables users to perform RingCT transactions, enhancing privacy by concealing the amount transferred.
