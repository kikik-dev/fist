# Fist

Fist is a Rust framework for building JSON APIs. It sits on top of **Axum** and **Tokio** and ships as a Cargo workspace: libraries you can reuse, a small CLI, and an example app in `src/` that shows how everything wires together.

---

## Get the example app running

If you already have Rust and a C compiler set up (see **First-time setup** if not), these are the only commands you need:

```bash
git clone https://github.com/kikik-dev/fist.git
cd fist
cargo build --workspace
cargo run
```

The last command starts the sample application (`my-fist-app`). When it is listening, open another terminal and hit your routes with `curl`, a REST client, or similar.

---

## First-time setup (new computer)

Follow these steps once on a machine where you have not built this project before.

### Step 1: Install Git

You need Git to clone the repository.

- Download: [https://git-scm.com/downloads](https://git-scm.com/downloads)

### Step 2: Install Rust (rustup)

Rust and Cargo come from the official installer:

1. Open **https://rustup.rs/** in your browser.
2. Follow the instructions for Windows, macOS, or Linux.
3. Accept the **stable** toolchain when asked (that is the right default for this project).
4. **Close and reopen** your terminal, then run:

   ```bash
   rustc --version
   cargo --version
   ```

   If both commands print a version, Rust is installed correctly.

You can install rustup **before** or **after** cloning Fist; order does not matter.

### Step 3: Install a C/C++ toolchain (needed for builds)

Some dependencies compile small pieces of C or C++ code (for example TLS, SQLite, and the memory allocator). If this step is missing, `cargo build` often fails with a confusing linker or compiler error.

Pick your system:

- **Windows:** Install [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/). During setup, enable the workload **Desktop development with C++** (includes MSVC and the Windows SDK). When you build Fist, use a terminal where those tools are on the PATH (for example **Developer PowerShell for Visual Studio**), or a normal terminal if the installer added the tools globally.

- **macOS:** Open Terminal and run:

  ```bash
  xcode-select --install
  ```

  Accept the prompt to install the **Command Line Tools**.

- **Linux (Ubuntu / Debian-style):** Run:

  ```bash
  sudo apt update
  sudo apt install build-essential pkg-config
  ```

  On Fedora, Arch, or other distros, install your equivalent of a compiler and “development headers” package group if a build fails.

### Step 4: Check that Fist builds

From the folder where you cloned the repo:

```bash
cargo build --workspace
```

If that finishes without errors, your machine is ready. You can go back to **Get the example app running** anytime.

### Optional extras (only if you need them)

- **cargo-watch** — Lets the CLI run `fist serve --watch` so the server restarts when files change. Install with:

  ```bash
  cargo install cargo-watch
  ```

- **k6** — Only if you want to run the bundled `load-test.js` against a running server. Installation: [https://k6.io/docs/get-started/installation/](https://k6.io/docs/get-started/installation/)

---

## Using the `fist` command

After you clone the repo, you **do not** yet have a `fist` command in the terminal. You choose one of the two approaches below.

### Option A: Install `fist` once (recommended if you use it often)

From the **root** of the cloned repository:

```bash
cargo install --path fist-cli
```

Wait for the build to finish. Then open a **new** terminal window and run:

```bash
fist --help
```

If that works, you can use `fist` from any directory (not only inside this repo). Cargo usually puts the binary in:

- Windows: `%USERPROFILE%\.cargo\bin`
- macOS / Linux: `~/.cargo/bin`

The rustup installer normally adds that folder to your PATH automatically. If `fist` is “not recognized”, add that folder to your PATH manually, then open a new terminal.

### Option B: No install — run through Cargo

Stay in the repository root and prefix commands like this:

```bash
cargo run -p fist-cli -- --help
cargo run -p fist-cli -- serve
cargo run -p fist-cli -- build --release
```

Use the same pattern for any `fist` subcommand: everything after `--` is passed to the CLI.

### Commands you will use often

If you installed `fist` (Option A), you can type these directly. If you use Option B, replace `fist` with `cargo run -p fist-cli --`.

```bash
fist serve              # run the app (debug)
fist serve --watch      # same, but reload on save (needs cargo-watch)
fist build --release    # release build of the workspace
```

The `new` and `make` subcommands exist for scaffolding; some flows still print placeholders instead of writing files. See `fist-cli/src/main.rs` for the current behavior.

---

## What is in this repository?

| Folder | What it is |
|--------|------------|
| `fist-core` | Framework core: routing, middleware, shared types. |
| `fist-macros` | Macros used by handlers and DTOs. |
| `fist-cli` | The `fist` command-line tool. |
| `src/` | Example application (routes, services, models). |

---

## What Fist helps with

- HTTP routing and handlers on Axum.
- Request validation via derive macros (see `fist-macros` and the example DTOs).
- Optional JWT-related middleware in `fist-core`.
- Example database code using **SeaORM** and **SQLite** for local development (you can change the backend for production).
- The sample app enables **mimalloc** as the global allocator on supported platforms.

---

## Performance

Any throughput or latency numbers depend on your machine, OS, Rust version, and how you load the server. Treat informal benchmarks as hints only, and measure on your own hardware or staging environment before you plan capacity.

---

## Contributing

Issues and pull requests are welcome. Small, focused changes that match the existing style are easiest to review.

---

## License

This project is released under the MIT License. See the `LICENSE` file for the full text.
