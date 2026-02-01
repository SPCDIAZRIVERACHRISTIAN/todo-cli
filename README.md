# 📝 todo-cli

A simple, fast, and persistent command-line todo application written in **Rust**.

Built as a learning project focused on **clean architecture**, **separation of concerns**, and **explicit state management**.

---

## ✨ Features

- Add, list, delete, and complete tasks
- Optional persistent storage via file
- Clean CLI interface using `clap`
- No hidden state — explicit in-memory vs persistent behavior
- Single-binary, zero dependencies at runtime

---

## 📦 Installation

### From source

```bash
git clone https://github.com/<your-username>/todo-cli.git
cd todo-cli
cargo install --path .
```

Make sure `~/.cargo/bin` is in your `$PATH`.

---

## 🚀 Usage

### Add a task (in memory)

```bash
todo add "buy milk"
```

> Note: in-memory tasks do not persist between runs.

---

### Add a task with persistence

```bash
todo --file tasks.txt add "buy milk"
```

---

### List tasks

```bash
todo --file tasks.txt list
```

---

### Mark a task as done

```bash
todo --file tasks.txt done 1
```

---

### Delete a task

```bash
todo --file tasks.txt delete 1
```

---

## 🗂 File Format

Tasks are stored as plain text, one per line:

```
<id>|<title>|<done>
```

Example:

```
1|buy milk|false
2|walk dog|true
```

This makes the storage:
- human-readable
- easy to inspect or edit
- future-proof for migrations

---

## 🧠 Design Notes

- **Tasks are pure domain data** — no I/O in the core logic
- **Storage is optional and explicit**
- **IDs are derived from state**, not generated via global counters
- CLI flags only affect orchestration, not business logic

This mirrors how real-world system CLIs behave (`git`, `cargo`, `docker`).

---

## 🛠 Tech Stack

- Rust
- [`clap`](https://crates.io/crates/clap) for CLI argument parsing

---

## 📌 Roadmap

- [ ] Default config file (`~/.config/todo/tasks.txt`)
- [ ] Colored output
- [ ] Sorting / filtering
- [ ] Export formats (JSON / CSV)
- [ ] REPL mode

---

## 📜 License

MIT
