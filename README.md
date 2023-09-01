# Equation Microservices

### Deps
- Rust + Cargo - I used rust version 1.72 but it likely runs fine on versions back to 1.64 which is required by [tonic](https://github.com/hyperium/tonic) - use [rustup](https://rustup.rs/) if you don't have it installed 
- [Protobuf Compiler](https://grpc.io/docs/protoc-installation/) - this is used to compile GPRC contracts between our micoservices and generate code via [tonic](https://github.com/hyperium/tonic)

###
Running via Shell
- Run all background microservices by running ```sh
chmod +x ./init-services.sh
./init-services.sh
  ```
- Run tests while services are still up in a new terminal session with ```cargo test```
- Run our test application with ```cargo run --package orchestrator``` - notice the main process will block after evaluation. Ctrl+C will stop the program and terminate downstream microservice nodes
