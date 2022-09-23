# Mosquitto Auth0 Plugin


### Requirements:
- Docker CE
- Rust LTS
```bash
asdf plugin-add rust https://github.com/code-lever/asdf-rust.git \
&& asdf install rust latest \
&& asdf global rust latest
```
- LLVM CLang
```bash
sudo apt install llvm-dev libclang-dev clang
```
- Mosquitto, Mosquitto SDK
```bash
sudo apt install mosquitto-dev libmosquitto-dev
```

### Run the example
- If all of the requirements was accomplish use the Makefile docker command to run the example
```bash 
make docker 
```

### Referencies:
- [Embedded Rust](https://docs.rust-embedded.org/book/interoperability/rust-with-c.html)
- [Mosquitto Plugin Documentation](https://mosquitto.org/api/files/mosquitto_plugin-h.html)
- [Rust JWT Plugin](https://github.com/wiomoc/mosquitto-jwt-auth)

### Exemples
- [Rust Mosquitto Plugin](https://github.com/TotalKrill/mosquitto_plugin)