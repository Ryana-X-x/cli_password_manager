# Passlock - CLI Password Manager

## Overview
Passlock is a Rust-based command-line password manager that securely encrypts and stores passwords using AES-256-GCM encryption. It enables users to add, retrieve, update, and delete passwords protected by a master password.

## Features
- **AES-256-GCM Encryption**: Ensures strong security with randomly generated nonces.
- **Master Password Protection**: Derives a secure encryption key using BLAKE3 hashing.
- **Secure Password Storage**: Stores encrypted credentials in a JSON file.
- **User-friendly CLI Interface**: Provides an intuitive command-line experience.

## Installation
Ensure you have Rust installed. If not, install it using:
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then, clone the repository and build the project:
```sh
git clone https://github.com/yourusername/passlock.git
cd passlock
cargo build --release
```

Run the application:
```sh
target/release/passlock
```

## Usage
1. **Launch Passlock** and enter a master password:
   ```sh
   ./passlock
   ```
2. **Choose an option**:
   - `1` Add Password
   - `2` Retrieve Password
   - `3` Update Password
   - `4` Delete Password
   - `5` Exit

3. **Follow on-screen prompts** to enter site/service names and passwords.

## Encryption & Security
- Uses **AES-256-GCM** with a unique nonce for each password.
- Master password is **never stored**; instead, a **BLAKE3-derived key** is used.
- Data is stored in `passwords.json` with encrypted values.

## Future Enhancements
- **Password generator** for creating strong passwords.
- **Cross-platform sync** via encrypted cloud storage.
- **Configurable storage locations**.

## License
Passlock is licensed under the MIT License. Feel free to contribute and improve!

## Contributing
1. Fork the repository.
2. Create a feature branch.
3. Submit a pull request with detailed changes.

For any issues, open a discussion on GitHub.

---
**Secure your passwords with Passlock! 🔒**

