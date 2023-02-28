## Getting Started

Getting started with the Rust demo.

### Prerequisites

- Install Rust from https://www.rust-lang.org/tools/install
- Install Docker from https://www.docker.com/
- (Optional: Install Postman from https://www.postman.com/)

### Installation

1. Clone the repo
   ```sh
   git clone git@github.com:torsrex/stoltit-rust-demo.git
   ```
2. Start the example Postgres Database
   ```sh
   docker compose up
   ```
3. Build and run the API
   ```sh
   cargo run
   ```
   <!-- USAGE EXAMPLES -->

## Usage

- Fetch all persons using a GET request to `localhost:8000/api/persons`
- Build without running using `cargo build`
- Format using `cargo fmt`
- Lint and autofix errors using `cargo clippy --fix -- -W clippy::pedantic -W clippy::nursery -W clippy::unwrap_used`
- Install Bacon using `cargo install bacon` and run bacon using `bacon`
