# hashstorage-cli

**hashstorage-cli** is a JS library that implements a convenient API to interact with a hashstorage backend (https://github.com/fomalhaut88/hashstorage). It includes a number of high level functions to create key pairs, encrypt, decrypt and sign data and request the hashstorage backend through its REST API. The mission of the library is to help your project provide the better security on the client side. As far as **hashstorage-cli** is written in Rust and compiled into WASM, it supports asynchronous import.

A simple example:

```javascript
(async () => {
    // Import the library
    const hsc = await import('hashstorage-cli')

    // Define a hashstorage backend API object
    const api = hsc.Api.new("https://hashstorage-cloud.com")

    // Get version on hashstorage backend
    const version = await api.getVersion()
    console.log(version)
})()
```

## Installation from source

1. Ensure you have `wasm-pack` installed (https://rustwasm.github.io/wasm-pack/installer/):

```
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
```

2. Clone `hashstorage-cli` repository:

```
git clone git+https://github.com/fomalhaut88/hashstorage-cli.git --depth 1
```

3. Build the project:

```
wasm-pack build --release --target bundler --out-dir hashstorage-cli-pkg
```

4. Install the package. For example:

```
npm i path/to/built/hashstorage-cli-pkg
```

## Usage example

As far as `hashstorage-cli` is powered by Rust,
the package must be imported asynchronously.

```javascript
const hsc = await import('hashstorage-cli')
```

To access the node, there is API object.

```javascript
const api = hsc.Api.new("https://hashstorage-cloud.com")
```

Every interaction through API is implemented as a promise.

```javascript
const version = await api.getVersion()
const groups = await api.getGroup(
    "F97CF0EA9BA1C36BE29045A14AAC32ED" +
    "9ECD8D67A9D6823D623E161B2600ED3B" +
    "4D3FA95A1580FED6068BD67013C99052" +
    "4DCCE132350EAC38948E3E15BC3E1E60"
)
const keys = await api.getKeys(
    "F97CF0EA9BA1C36BE29045A14AAC32ED" +
    "9ECD8D67A9D6823D623E161B2600ED3B" +
    "4D3FA95A1580FED6068BD67013C99052" +
    "4DCCE132350EAC38948E3E15BC3E1E60",
    "mygroup"
)
```

Profile stores public and private keys to sign and check blocks.
Also public key is a user identifier in hashtorage.
Profile object can be created from application id, username and password.

```javascript
const profile = hsc.Profile.new("appidstring", "alex", "Qwerty123")
const publicKey = profile.publicKey()
```

There are methods to access hashstorage instance.

```javascript
const groups = await profile.groups(api)
const keys = await profile.keys(api, "mygroup")
const blockJson = await profile.getBlockJson(api, "mygroup", "mykey")
```

Profile can be saved and loaded in `localStorage`.

```javascript
const existsInLocalStorage = hsc.Profile.exists()
const profile = hsc.Profile.load()
const isValid = profile.check()
profile.save()
profile.clear()
```

With profile it is possible to sign and check blocks.

```javascript
const signature = profile.buildSignature(...)
const isCorrect = hsc.Profile.checkSignature(publicKey, ...)
```

Block represents a wrapper for data to store in hashstorage. It contains the
identifier (public key, group and key), version, signature and data itself.
It can be created from public key, group and key or from block JSON provided
by `profile.getBlockJson`.

```javascript
const block1 = hsc.Block.new(profile.publicKey(), "mygroup", "mykey")
const block2 = hsc.Block.fromBlockJson(blockJson)
```

Block attributes can be extracted by their names.

```javascript
const publicKey = block.publicKey()
const group = block.group()
const key = block.key()
const version = block.version()
const data = block.data()
const signature = block.signature()
```

Some data can be modified:

```javascript
block.setData("new data")
block.setVersion(123)
```

There are methods to work with the signature:

```javascript
const isSigned = block.isSigned()
block.sign(profile)
block.clearSignature()
```

There are two similar methods to save block in hashstorage.
`update` just requests with the current data.
`save` increments the version, builds signature and
after that calls `update`.

```javascript
await block.update(api)
await block.save(api, profile)
```

## How to use hashstorage-cli in JS worker

The work with JS workers has some specials because there is no access
to the window object, to localStorage and other familiar things.
Thus the project must be compiled in a different way,
not as an ordinary package.

```
wasm-pack build --release --target no-modules --no-typescript --out-dir hashstorage-cli-wasm
```

The example of worker is following:

```javascript
/* worker.js */

importScripts("hashstorage-cli-wasm/hashstorage_cli.js?v=0.0")

(async () => {
    await wasm_bindgen("hashstorage-cli-wasm/hashstorage_cli_bg.wasm")
    hsc = wasm_bindgen;

    self.addEventListener('message', e => {
        console.log("Message:", e)
        let result = e.data[0] + e.data[1];
        postMessage(result)
    })
})()
```

The example of create a worker:

```javascript
let myWorker = new Worker('worker.js')

myWorker.addEventListener('message', e => {
    console.log(e)
}, false)

// In some cases it must be called is a while (100 ms for example)
// after creating the worker, because before the worker can handle the message
// it must be loaded that is done asynchronously.
// For example, it can be called with setTimeout.
myWorker.postMessage([42, 23])
```

**Note!** You are unable to make HTTP requests inside of a JS worker, because this operation requires the window object to exist. So you are unable to use the methods of the API object to a hashstorage backend. A solution is you can send the data from the worker with `postMessage` and perform the HTTP request in the main script.
