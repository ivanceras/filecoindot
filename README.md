filecoindot
======================

[<img alt="github" src="https://img.shields.io/badge/github-ChainSafe/filecoindot-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/ChainSafe/filecoindot)
[<img alt="build status" src="https://img.shields.io/github/workflow/status/ChainSafe/filecoindot/CI/main?style=for-the-badge" height="20">](https://github.com/ChainSafe/filecoindot/actions?query=branch%3Amain)
[<img alt="license" src="https://img.shields.io/badge/License-LGPL%20v3-blue?style=for-the-badge" height="20">](http://www.gnu.org/licenses/lgpl-3.0)

A Substrate pallet to bridge from Filecoin to any blockchain built using the [Substrate](https://www.substrate.io/)
framework, including Polkadot parachains.

## How to integrate the Filecoin bridge pallet into a runtime?

#### 0. configure `filecoindot` into your node

Here we need to configure filecoindot into our runtime first, see [substrate-node-example](./substrate-node-example/README.md) 
for detail.


#### 1. set signer account to your node

Generate an account with [subkey](https://github.com/paritytech/substrate/tree/8b95e236582c209a1676d75a1db61a4916faabf5/bin/utils/subkey) for our offchain worker first

```
$ subkey generate
Secret phrase `brief outside human axis reveal boat warm amateur dish sample enroll moment` is account:
  Secret seed:      0x4ebb14295f95e62a865a457629a8e6d96ef5f3cf1896a9624d2e91e09f4cdc65
  Public key (hex): 0x0676a4b19c66b31e12d15fe31ccbc775d3d2cda6e1c8686e395118f808eaa118
  Account ID:       0x0676a4b19c66b31e12d15fe31ccbc775d3d2cda6e1c8686e395118f808eaa118
  SS58 Address:     5CDBPWWtnLTqSUSNvB5BGYMs23Vs8dDHwWpjWpebBRErZM9W
```

and then, post your account to the node

```
$ curl -X POST -vk 'http://localhost:9933' -H "Content-Type:application/json;charset=utf-8" \
  -d '{
    "jsonrpc":2.0,
    "id":1,
    "method":"author_insertKey",
    "params": [
      "fdot",
      "0x4ebb14295f95e62a865a457629a8e6d96ef5f3cf1896a9624d2e91e09f4cdc65",
      "0x0676a4b19c66b31e12d15fe31ccbc775d3d2cda6e1c8686e395118f808eaa118"
    ]
  }'
```

#### 2. set filecoin rpc endpoint to your node


```
curl -X POST -H "Content-type: application/json"  http://localhost:9933 -d '
{
  "method": "filecoindot_setRpcEndpoint",
  "jsonrpc": "2.0",
  "id": 0,
  "params": ["http://user:pass@infura.io"]
}
'
```

If you can see the logs below in your terminal

```
2021-10-31 21:58:55 Running in --dev mode, RPC CORS has been disabled.
2021-10-31 21:58:55 Substrate Node
2021-10-31 21:58:55 ✌️  version 3.0.0-9e5d007-aarch64-macos
2021-10-31 21:58:55 ❤️  by Substrate DevHub <https://github.com/substrate-developer-hub>, 2017-2021
2021-10-31 21:58:55 📋 Chain specification: Development
2021-10-31 21:58:55 🏷 Node name: fuzzy-doctor-8068
2021-10-31 21:58:55 👤 Role: AUTHORITY
2021-10-31 21:58:55 💾 Database: RocksDb at /Users/clearloop/Library/Application Support/node-template/chains/dev/db
2021-10-31 21:58:55 ⛓  Native runtime: node-template-100 (node-template-1.tx1.au1)
2021-10-31 21:58:55 Using default protocol ID "sup" because none is configured in the chain specs
2021-10-31 21:58:55 🏷 Local node identity is: 12D3KooWPY6sbvMAryQkXRDKiApkd75Ak8x7cbgEqtiEzFxb5kQF
2021-10-31 21:58:55 could not parse an IP from hosts file
2021-10-31 21:58:57 📦 Highest known block at #168
2021-10-31 21:58:57 〽️ Prometheus exporter started at 127.0.0.1:9615
2021-10-31 21:58:57 Listening for new connections on 127.0.0.1:9944.
2021-10-31 21:59:00 🙌 Starting consensus session on top of parent 0x6b98f8856f41abfb45da878188aaff1419c1cb6bbb8a56a346e9f1f39613a1a4
2021-10-31 21:59:00 🎁 Prepared block for proposing at 169 [hash: 0x9f3941c44fdacc7f240b0471c97cd61e5010377619501285999ab0ff4d0edbbc; parent_hash: 0x6b98…a1a4; extrinsics (1): [0x1dd1…1b8b]]
2021-10-31 21:59:00 🔖 Pre-sealed block for proposal at 169. Hash now 0x53480ec65b6b2c13126a96c62089af0244a570ba961efdd0d377a44df12a5a5b, previously 0x9f3941c44fdacc7f240b0471c97cd61e5010377619501285999ab0ff4d0edbbc.
2021-10-31 21:59:00 ✨ Imported #169 (0x5348…5a5b)
2021-10-31 21:59:00 bootstrap filecoindot ocw with filecoin rpc endpoint http://user:pass@infura.io
```

the filecoindot offchain worker has been set up!


#### 3. set the the offchain worker's account as relayer

_TODO_


#### 4. Full Example

See [the example runtime](./substrate-node-example/runtime/src/lib.rs) for a full example showing how to integrate the pallet into
a substrate runtime.

# ChainSafe Security Policy

## Reporting a Security Bug

We take all security issues seriously, if you believe you have found a security issue within a ChainSafe project please
notify us immediately. If an issue is confirmed, we will take all necessary precautions to ensure a statement and patch
release is made in a timely manner.

Please email us a description of the flaw and any related information (e.g. reproduction steps, version) to
[security at chainsafe dot io](mailto:security@chainsafe.io).
