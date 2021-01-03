# locShare

# Working principles:

## Adding new node to Local Sharing Network

A - first station with keys
U - user
N - new station
ic - invitation code
P - public key of A
p - private key of A
S - group symmetric key
g - diffie-hellman group
a - diffie-hellman key for A
n - diffie-hellman key for B

1. A ---[ic]---> U,                      [ic - random invitation code]
2. A ---[X1]---> broadcast -----> N,    [X1 - (P,g) encrypted with `ic`]
3. U ---[ic]---> N
4. N ---[X2]---> A,                     [X2 - g^a encrypted with `P`]
5. A ---[X3]---> N,                     [g^b, X3 - symmetric key encrypted with g^(ab) and p]

1. `A` generates **random invitation code** and shows it to user `U`
2. `A` sends encrypted **random invitation code** over broadcast
3. User types invitation code `ic` into new station `N`
4. Machine `N` sends its DH part to `A`
5. Machine `A` sends secret key encrypted with common DH key and encrypts it with its private key `p`. `N` decrypts message with `P` and common DH key.
6. New machine `N` should have the same symmetric key as `A`

## Messages

1. Broadcasted message
```
|-----------------------------------------------------------|
| - Public key (encrypted with Invitation Code)             |
| - Diffie-hellman group (encrypted with Invitation Code)   |
| - crc32                                                   |
|-----------------------------------------------------------|
```