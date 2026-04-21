# ⚠️ Errors_Custom (Anchor)

This project demonstrates how to implement **custom error handling in Solana programs using Anchor**. It showcases how to enforce constraints and return meaningful errors using `require!` and `err!` macros.

---

## 📌 Overview

Error handling is critical in Solana smart contracts to ensure **data validation, security, and predictable execution**.

This project includes:

* Range validation using `require!`
* Manual error triggering using `err!`
* Custom error definitions using `#[error_code]`
* Clean and readable error messages

---

## 🛠️ Tech Stack

* **Rust** – core programming language
* **Solana** – blockchain platform
* **Anchor Framework** – for writing Solana programs

---

## ⚙️ Features

* Enforces numeric constraints on input
* Returns descriptive custom errors
* Demonstrates both validation and forced failure cases
* Uses Anchor idiomatic error handling

---

## 📂 Project Structure

```
Errors_Custom/
│── programs/
│   └── day_4/
│       └── src/
│           └── lib.rs     # Main program logic
│── Anchor.toml
│── Cargo.toml
│── README.md
```

---

## 🚀 Program Functions

### 1️⃣ `limit_range`

Validates that the input `a` is within a specific range.

```rust
pub fn limit_range(_ctx: Context<LimitRange>, a: u64) -> Result<()> {
    require!(a >= 10, MyError::AisTooSmall);
    require!(a <= 100, MyError::AisTooBig);
    msg!("Result = {}", a);
    Ok(())
}
```

#### ✔️ Behavior:

* If `a < 10` → throws `AisTooSmall`
* If `a > 100` → throws `AisTooBig`
* If valid → prints the value

---

### 2️⃣ `func`

Always returns a custom error.

```rust
pub fn func(_ctx: Context<LimitRange>) -> Result<()> {
    msg!("Will this print?");
    return err!(MyError::AlwaysErrors);
}
```

#### ✔️ Behavior:

* Logs message: `"Will this print?"`
* Immediately fails with `AlwaysErrors`

---

## ❗ Custom Errors

Defined using Anchor’s `#[error_code]`:

```rust
#[error_code]
pub enum MyError {
    #[msg("a is too small")]
    AisTooSmall,

    #[msg("a is too big")]
    AisTooBig,

    #[msg("Always errors")]
    AlwaysErrors,
}
```

---

## 🧠 Key Concepts

### 🔹 `require!` Macro

* Used for validation
* Stops execution if condition fails

### 🔹 `err!` Macro

* Manually returns an error
* Useful for forced failures or edge cases

### 🔹 Custom Error Codes

* Improve debugging
* Provide meaningful feedback
* Essential for production-grade smart contracts

---

## 🚀 Getting Started

### 1️⃣ Install Dependencies

* Rust
* Solana CLI
* Anchor CLI

---

### 2️⃣ Build the Program

```bash
anchor build
```

---

### 3️⃣ Test the Program

```bash
anchor test
```

---

## 🧠 What I Learned

* Writing custom errors in Anchor
* Using `require!` for validation
* Using `err!` for manual error handling
* Structuring safer and more reliable smart contracts

---

## 🎯 Future Improvements

* Add frontend interaction (React + Solana Wallet)
* Write detailed test cases
* Add more complex validation logic
* Integrate with a real dApp

---

## 📜 License

This project is open-source under the **MIT License**.

---

## 👨‍💻 Author

**Akshat Upadhyay**
Aspiring Blockchain Developer 🚀
