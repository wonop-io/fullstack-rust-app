# Fullstack Rust App: Ethereum Wallet Project

This repository is a companion to the book [Mastering Rust Full Stack: Building Modern Web and Desktop Applications with Yew and Axum](https://wonop.gumroad.com/l/mastering-rust-fullstack).

This repository hosts the full stack project project for building a fullstack Ethereum wallet application using Rust.
The project demonstrates how to create a secure wallet, interact with the Ethereum blockchain, and build a responsive user interface with Yew and Tailwind CSS.
It is not meant to be complete but rather demonstrate.

A secure Ethereum wallet built with Rust, Yew, and WebAssembly. This project demonstrates full-stack Rust development with features like:

- Interactive UI built with Yew and Tailwind CSS
- Secure user authentication
- Encrypted wallet storage
- Real-time transaction tracking
- Integration with Ethereum blockchain


## Prerequisites

Before running this project, ensure you have installed:

- Rust and Cargo (latest stable version)
- Node.js (for Tailwind CSS)
- Docker (for Redis and PostgreSQL)
- Trunk (for serving WebAssembly)

## Getting Started

1. Clone the repository:
```bash
git clone https://github.com/yourusername/ethereum-wallet
cd ethereum-wallet
```

2. Start the database and Redis containers:
```bash
docker-compose up -d
```

3. Set up the database schema:
```bash
sqlx database create
sqlx migrate run
```

4. Start the development servers:

Terminal 1 (Backend):
```bash
cargo run --bin backend
```

Terminal 2 (Frontend):
```bash
cd app/frontend
trunk serve
```

Terminal 3 (Local Ethereum node):
```bash
anvil
```

## Project Structure

```text
ethereum_wallet/
├── app/
│   ├── frontend/      # Yew frontend application
│   └── backend/       # Axum backend server
├── wallet/            # Wallet functionality
│   ├── wallet_screens/
│   ├── wallet_app/
│   └── wallet_api/
├── auth/              # Authentication system
│   ├── auth_screens/
│   ├── auth_app/
│   └── auth_api/
└── app_config/        # Shared configuration
```

## Development

The application will be available at:
- Frontend: http://localhost:8080
- Backend API: http://localhost:8000
- Local Ethereum node: http://localhost:8545

## Environment Variables

Create a .env file in the root directory with:

```env
DATABASE_URL=postgres://postgres:password@localhost:5432/ethereum_wallet
REDIS_URL=redis://localhost:6379
JWT_SECRET=your_jwt_secret
```

## Features

- Secure wallet creation and import
- Transaction sending and receiving
- Real-time balance updates
- Transaction history tracking
- QR code generation for receiving funds
- Password-protected private keys
- Session management with Redis
- Responsive UI with Tailwind CSS

## Security Notes

- All private keys are encrypted before storage
- Passwords are hashed using Argon2
- Authentication tokens are managed securely through HTTP-only cookies
- CORS is properly configured for development

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE.md file for details
