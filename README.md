# locShare

# Working principles:

## Adding new node to Local Sharing Network

E - existing station with keys
U - user
N - new station
ic - invitation code
P - public key of E
p - private key of E
S - symmetric key of our devices
g - diffie-hellman group
a - diffie-hellman key for E
n - diffie-hellman key for B

1. E ...[ic]...> U,                     [ic - random invitation code passed securely out of band]
2. U ---[ic]---> N                      [ user writes random invitation code to new device and clicks join request ]
3. N ---[X1]---> broadcast -----> E,    [X1 - random ephemeral key (`eph`) encrypted with `ic`]
4. E ---[X2]---> N,                     [X2 - group key `S` encrypted with decrypted `eph` key ]

1. `E` generates **random invitation code** and shows it to user `U`
2. user `U` writes **random invitation code** to new device `N`
3. Device `N` sends ephemeral key `eph` encrypted with `ic` over broadcast to `E`
4. Machine `E` sends back secret key `S` encrypted with `eph`
5. New machine `N` should have the same symmetric key as `E`

##

## Messages

1. Broadcasted message

```
|-----------------------------------------------------------|
| - EncryptedMsg (encrypted with Invitation Code)           |
| - crc32                                                   |
|-----------------------------------------------------------|
```

