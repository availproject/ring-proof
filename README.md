## Contents

* [`common`](common) provides infrastucture for creating plonk-like proofs.  
* [`ring`](ring) for a vector commitment to a list of public keys, and a Pedersen commitment to one of the secret keys, implements a zk proof of knowledge of the blinding factor for the Pedersen commitment, and the position of the corresponding public key in the list.

## TODOs
* Fix FiatShamir:
  - add instance to the transcript,
  - verifier uses Fiat-Shamir rng to batch verify the pairings,
  - remove test_rng from not_test,
* Verifier evaluates selectors efficiently
* Add zk
* Batch verification
* Batch proving