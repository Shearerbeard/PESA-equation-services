# Equation Microservices

### Deps
- Rust + Cargo - I used rust version 1.72 but it likely runs fine on versions back to 1.64 which is required by [tonic](https://github.com/hyperium/tonic) - use [rustup](https://rustup.rs/) if you don't have it installed 
- [Protobuf Compiler](https://grpc.io/docs/protoc-installation/) - this is used to compile GPRC contracts between our micoservices and generate code via [tonic](https://github.com/hyperium/tonic)

###
Running via Shell
- Run all background microservices by running
```sh
chmod +x ./init-services.sh
./init-services.sh
```
- Run tests while services are still up in a new terminal session with ```cargo test```
- Run our test application with ```cargo run --package orchestrator``` - notice the main process will block after evaluation. Ctrl+C will stop the program and terminate downstream microservice nodes

###
Closing Thoughts / TODOS
- Docker: For fun I had the strech goal of having the concurrent services easily runnable with docker compose but realized I didn't have time to mess with the port forwarding as each node also needs a TCP connection for GRPC as a client to other possible nodes. Easily solveable but I simply ran out of time.
- DRY: Each Services server.rs implementation of MathASTEvaluator<E> is nearly identical, same goes for the get_x_client functions defined on the service structs. I could have abstracted these out in the shared equations module by simply defining some trait interfaces for each service (Example `trait WithAdderClient { async fn get_add_client(&self) -> Result<SubtractorClient<Channel>, Error>;  }`). Again this didn't end up fitting my time budget. Similar story for the async recursive AST parser in each module (try_from_ast). The function try_from_ast could have easily been implemented with a generic signature like `async fn try_from_ast(service: &impl MathASTEvaluator, ast: MathAST) -> Result<MathAST, Error>;` and be defined once in our shared lib module.
- Parser: I built my implementation of this challenge as a distributed async recursive iteration of a defined AST. I had a stretch goal of writing a parser to derive this AST from an equation string but my time was spent on more pressing priorities. Its also worth noting that this AST only supports a subset of mathmatical operations and does not handle order of operations - its a naive implemenation that only works with nested queries (like the example). I do belive my code could easily be upgraded at a later date to handle AST as a possible Vec and derive order of operations while maintaining most of the code I've already written.