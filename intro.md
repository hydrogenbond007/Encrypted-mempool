# Encrypted Mempools on Solana

Choose a homomorphic encryption scheme: To implement an [encrypted mempool](https://www.youtube.com/watch?v=fHDjgFcha0M&list=WL&index=1)  on Solana, you will first need to choose a homomorphic encryption scheme that is well-suited to the types of computations that will be performed on the encrypted data. Some possible options include the Paillier cryptosystem, the BGV scheme, or the CKKS scheme.

Encrypt transactions as they are added to the mempool: When a transaction is submitted to the mempool, it should be encrypted using the chosen homomorphic encryption scheme. This will require generating a public key that can be used to encrypt the transaction or a system of decentralized sequencer could use [this](https://0x0000.app/2023/01/22/solana-accounts-db.html) 

Store the encrypted transactions in a database: Encrypted transactions should be stored in a database that is optimized for efficient search and retrieval. Solana's built-in database, called "RocksDB", may be suitable for this purpose.

Process encrypted transactions: When a block is created, the transactions in the mempool must be decrypted in order to be included in the block. It could use [delyaed encryption](https://eprint.iacr.org/2020/638.pdf) in which the cipher text is decrypted itself at some point in time or [thereshold cryptography](https://en.wikipedia.org/wiki/Threshold_cryptosystem) but that needs a trusted party which could go offline causing further imlications. This can be accomplished by using the corresponding private key to decrypt the transactions. Once the transactions are decrypted, they can be processed as usual.

Consider performance and scalability: Performance and speed is important as its already hard for mev to reorder becuase of Solana's 400ms block time and slot times being highly efficient leaving very less for the whole shabang of mempool to occur.

## General Idea
MEV is one of the biggest consumers of Compute on solana nearly 70% according to the jito states while most of them are failed transactions that don't end up paying any kind of gas, they hurt the blockspace and regular user experience. 
While solana transactions are super fast there is still a risk of the validator front running the transactions and arbitrage the opportunity and making money of it. The ux is better because of PBS but still, the actors can collude if the reward is bigger than slashing.

One of the ways to solve this could be having a shared mempool of transactions that is totally encrypted kind alike what SUAVE is trying to build but we don't want to be a totally different state layer that has some issues of its own. We can build a mempool that can be used to fetch txns and make more efficient blocks while only the gas reward attached to the transaction is shared till it's included in the block. 

### Wormhole

Having better transaction orderflow will help in cross chain arbitrages and prevent bad actors from front-running maintaining the integrity of the bridge and allow private cross chain calls which will help with the censorship resistant of relayers.

Here is a more indepth guide and technical overview of the approach →  

https://github.com/hydrogenbond007/Encrypted-mempool

### Pros:

- Better Block-packaging as transactions are more focused on maximising their cu to gas ratio
    - 
    
    $$
    Compute_\ Units / Gas_\ used
    $$
    
- Protection against the front running and bad MEV
- Surge in cross-domain mev as the txns is encrypted, the opportunity wont be shared by anyone

### Cons:

- This may lead to centralisation of block production as proposers will only look for the highest reward
- Hard censorship of bad actors

We could have a small middleware rpc that or a fork of the jito searcher/bundler that uses Elgamal or some form of delayed encryption and which decrypts only after the txn is included in the block.

We could use either BLS or some other signature which would have a specific computation time required without the need for any trusted third party. ore info and the research paper here 

https://eprint.iacr.org/2023/189.pdf

<img width="850" alt="Screenshot 2023-06-01 at 12 34 02 PM" src="https://github.com/hydrogenbond007/Encrypted-mempool/assets/88841339/1799a43d-c5cc-4e75-bbb5-78c014a0a2bb">


NOTE: Solana Block times are 400ms which is one of the biggest hurdles in introducing PBS and complex transaction orderflows. because there is a chance if the we cant get the encryption to be fast enough all the encrypted txns could be dropped because the bundler might not be able to keep up. 

### Some previous research

https://mirror.xyz/madhavg.eth/HFeX2F9U6g7uSMx_A4rn9VmOgs59ZlDwrkYu_zS9aVc
https://mirror.xyz/madhavg.eth/1Gfa6tupaMVQ13-vEfchvllpgVKgOZONPO6o6hnUfEw
