First need to install lmdb : 

on mac os : 
'''bash
brew install lmdb
'''

then set up your env variable

The logic of the client is the following.
For the moment there is no consensus, the client is running using poa.
the client is divided in 4 parts: 
    - the listenner that is always listenning to the plasma contract on the root blockchain.
Based on events, the listenner can send tx to the mempool (in case of deposit or withdrawal)
    - the API, this is the equivalent of JSON RPC for our blokchain, for the moment it is a rest API.
it allows user to post tx, and get data from the network
    - the block builder, it's role is to create new block by looking at the tx in the mempool
    - the databases, they are 5 databases :
        - the mempool, this is where pending are stored
        - the utxo set, this is all the output
        - the transactions,
        - the blocks
        - the index, this allows to map a block number to a mapping -> facilitating API