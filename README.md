# rust-ztm
rust wrap of flomesh-io/ztm


# build

> [!IMPORTANT]
>
> Building `rust-ztm` requires `cmake`, `clang`, `nodejs`, `npm`, and the `rust toolchain`.
>
> Additionally, building with the `agent-ui` features requires that `vite` be installed.

Follow the steps below to build rust-ztm

```shell
git clone https://github.com/flomesh-io/rust-ztm.git
git submodule update --init --recursive
cargo build
```

# features
- [x] `agent-ui`: build with ztm agent's web ui，for development and testing purpose.
