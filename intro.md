# Encrypted Mempools on Solana

Choose a homomorphic encryption scheme: To implement an [encrypted mempool](https://www.youtube.com/watch?v=fHDjgFcha0M&list=WL&index=1)  on Solana, you will first need to choose a homomorphic encryption scheme that is well-suited to the types of computations that will be performed on the encrypted data. Some possible options include the Paillier cryptosystem, the BGV scheme, or the CKKS scheme.

Encrypt transactions as they are added to the mempool: When a transaction is submitted to the mempool, it should be encrypted using the chosen homomorphic encryption scheme. This will require generating a public key that can be used to encrypt the transaction or a system of decentralized sequencer could use [this](https://0x0000.app/2023/01/22/solana-accounts-db.html) 

Store the encrypted transactions in a database: Encrypted transactions should be stored in a database that is optimized for efficient search and retrieval. Solana's built-in database, called "RocksDB", may be suitable for this purpose.

Process encrypted transactions: When a block is created, the transactions in the mempool must be decrypted in order to be included in the block. It could use [delyaed encryption](https://eprint.iacr.org/2020/638.pdf) in which the cipher text is decrypted itself at some point in time or [thereshold cryptography](https://en.wikipedia.org/wiki/Threshold_cryptosystem) but that needs a trusted party which could go offline causing further imlications. This can be accomplished by using the corresponding private key to decrypt the transactions. Once the transactions are decrypted, they can be processed as usual.

Consider performance and scalability: Performance and speed is important as its already hard for mev to reorder becuase of Solana's 400ms block time and slot times being highly efficient leaving very less for the whole shabang of mempool to occur.


