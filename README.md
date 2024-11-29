# Clutch-Node

Clutch-Node is a blockchain-based ridesharing platform that aims to improve urban mobility by leveraging blockchain technology to create a decentralized, efficient, and secure system for ridesharing.

## Features
- **Decentralized System**: Eliminates intermediaries, allowing users to connect directly.
- **Secure Transactions**: Utilizes blockchain technology to ensure the security and privacy of all transactions.
- **User Empowerment**: Provides users with more control over their ridesharing experiences.
- **Eco-friendly Options**: Encourages the use of electric and hybrid vehicles to reduce carbon footprint.


### Prerequisites
- Docker
- Docker Compose

## Running the Project Locally

To get started with Clutch-Node, follow these steps:

- Clone the repository:
git clone https://github.com/MehranMazhar/clutch-node

- Start the application:
- cd clutch-node
- cargo run -- --env node1

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
The project is built using Docker to ensure a consistent environment. The provided Dockerfile handles all dependencies and builds the project in release mode.

- docker build -t clutch-node .

### Running Multiple Nodes on Different Networks
To run multiple nodes, you need to specify different networks and ports:

- docker network create clutch-network1
- docker network create clutch-network2
- docker network create clutch-network3
- docker-compose up node1
- docker-compose up node2
- docker-compose up node3