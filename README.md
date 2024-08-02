# Clutch-Node

Welcome to Clutch-Node, the innovative blockchain-based ridesharing platform. Our project is committed to improving urban mobility by leveraging the power of blockchain technology to create a decentralized, efficient, and secure system for ridesharing.

## Introduction

Clutch-Node is designed to revolutionize the way we think about ridesharing. By integrating blockchain technology, we aim to provide a more transparent, fair, and user-friendly experience compared to traditional ridesharing services.

## Features

- Decentralized System: Eliminates the need for intermediaries, allowing users to connect directly.
- Secure Transactions: Utilizes blockchain technology to ensure the security and privacy of all transactions.
- User Empowerment: Provides users with more control over their ridesharing experiences.
- Eco-friendly Options: Encourages the use of electric and hybrid vehicles to reduce carbon footprint.

## Running the Project Locally

To get started with Clutch-Node, follow these steps:

- Clone the repository:
git clone https://github.com/MehranMazhar/clutch-node

- Install dependencies:
cd clutch-node

- Start the application:
cargo run -- --env node1

## Instarll Clang on windows
ECHO %LIBCLANG_PATH%
SET LIBCLANG_PATH=C:\Program Files\Microsoft Visual Studio\2022\Enterprise\VC\Tools\Llvm\x64\bin

## Contributing

Contributions are what make the open-source community such an amazing place to learn, inspire, and create. Any contributions you make are **greatly appreciated**.

## License

Distributed under the MIT License. See `LICENSE` for more information.

## Contact

If you have any questions or comments, please feel free to contact us at mehran.mazhar@gmail.com.

## Dcoker

### Building the Project
- The project is built using Docker to ensure a consistent environment. The provided Dockerfile handles all dependencies and builds the project in release mode.
- docker build -t clutch-node .

### Running the Node
You can run the node using Docker. Below are examples of running multiple nodes with different configurations.

- docker run --name clutch-node-container-node1 -it --rm -p 8081:8081 clutch-node node1  
- docker run --name clutch-node-container-node2 -it --rm -p 8082:8082 clutch-node node2  
- docker run --name clutch-node-container-node3 -it --rm -p 8083:8083 clutch-node node3

### Running Multiple Nodes on Different Networks
- docker network create clutch-network1
- docker network create clutch-network2
- docker network create clutch-network3
- docker run --name clutch-node-container-node1 -it --rm --network clutch-network1 -p 8081:8081 clutch-node node1  
- docker run --name clutch-node-container-node2 -it --rm --network clutch-network2 -p 8082:8082 clutch-node node2  
- docker run --name clutch-node-container-node3 -it --rm --network clutch-network3 -p 8083:8083 clutch-node node3