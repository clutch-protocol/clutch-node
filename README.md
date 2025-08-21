# Clutch-Node

![Alpha](https://img.shields.io/badge/status-alpha-orange.svg)
![Experimental](https://img.shields.io/badge/stage-experimental-red.svg)
![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)
![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)

> ⚠️ **ALPHA SOFTWARE** - This project is in active development and is considered experimental. Use at your own risk. APIs may change without notice.

Clutch-Node is a blockchain-based ridesharing platform that aims to improve urban mobility by leveraging blockchain technology to create a decentralized, efficient, and secure system for ridesharing.

## Features
- **Decentralized System**: Eliminates intermediaries, allowing users to connect directly.
- **Secure Transactions**: Utilizes blockchain technology to ensure the security and privacy of all transactions.
- **User Empowerment**: Provides users with more control over their ridesharing experiences.
- **Eco-friendly Options**: Encourages the use of electric and hybrid vehicles to reduce carbon footprint.

## Prerequisites
- Docker
- Docker Compose

## Running the Project Locally

To get started with Clutch-Node, follow these steps:

1. Clone the repository:
    ```bash
    git clone https://github.com/MehranMazhar/clutch-node
    cd clutch-node
    ```

2. Start the application:
    ```bash
    cargo run -- --env node1
    ```

## Installing Clang on Windows
Set the `LIBCLANG_PATH` environment variable:
```bash
ECHO %LIBCLANG_PATH%
SET LIBCLANG_PATH=C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\Llvm\x64\bin
```

## Contributing
Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## License
Distributed under the Apache License 2.0. See `LICENSE` for more information.

## Contact
If you have any questions or comments, please feel free to contact us at mehran.mazhar@gmail.com.

## Docker

### Building the Project
The project is built using Docker to ensure a consistent environment. The provided Dockerfile handles all dependencies and builds the project in release mode.

```bash
docker build -t clutch-node .
```

### Running Multiple Nodes on Different Networks
To run multiple nodes, you need to specify different networks and ports:

```bash
docker network create clutch-network1
docker network create clutch-network2
docker network create clutch-network3
docker-compose up node1
docker-compose up node2
docker-compose up node3
```