# rustlings_v6_solutions

My solutions for Rustlings v6.

I started with Rustlings version `5.x`, which used a different way to check the exercise's progression. The exercises also seems to have gone through some important changes, so I decided to keep the old solutions in a separate repo (https://github.com/markgreene74/rustlings_v5_solutions) and work the entire set again from scratch.

This repository uses [`pre-commit`](https://pre-commit.com/).

## pre work

- re-install `rustlings` following the instructions in the [Getting Started](https://github.com/rust-lang/rustlings?tab=readme-ov-file#getting-started) section of the README.
- check that version `6.x` is now installed:
    ```shell
    $ cargo install --list
    rustlings v6.1.0:
        rustlings
    ```
    or
    ```shell
    $ rustlings --version
    rustlings 6.1.0
    ```
- install `pre-commit`, see the [installation notes](https://pre-commit.com/#install)
    ```shell
    pip install pre-commit
    pre-commit install
    ```

## rustlings

```shell
cargo install rustlings
rustlings init
```

**NOTE**: as `rustlings` in this case is inside an outer repo, remove the `.git` directory that was automatically created.

```shell
rm -rf rustlings/.git
```

```shell
cd rustlings/
rustlings
```
