# Mosquitto Identity Plugin

[![ci](https://github.com/tointernet/mosquitto.identity.plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/tointernet/mosquitto.identity.plugin/actions/workflows/ci.yml) [![codecov](https://codecov.io/gh/tointernet/mosquitto.identity.plugin/branch/main/graph/badge.svg?token=X9L45YZD95)](https://codecov.io/gh/tointernet/mosquitto.identity.plugin)

In a IoT platform one of the most important thing is to ensure that each device can access only the resources that the device is allowed to access.
It's important because on the IoT platform we don't know where our devices are and who has them. So the basic security principle to reduce as much as we can
the access to our platform is extremely importante for IoT. With this in mind this plugin allow us to overwrite the basic connection authentication and create an ACL for the [Mosquitto Broker](https://mosquitto.org/). Each user will have a group
of Permissions called Roles and these permissions will granted or deny the access for each topic, with that we can managed the publish and the subscription for each topic.

# Table of content

- [Mosquitto Plugin](#mosquitto-plugin)
  - [Broker Startup](#broker-startup)
  - [Mosquitto Basic Auth Workflow](#mosquitto-basic-auth-workflow)
  - [Mosquitto ACL Workflow](#mosquitto-acl-workflow)
- [Requirements](#requirements)
- [Example](#run-the-example)

## Mosquitto Plugin

The Mosquitto broker give us a external API to build a custom plugin. This API is exposed in the [mosquitto_plugin.h](https://mosquitto.org/api/files/mosquitto_plugin-h.html).

### Broker Startup

When the broker start to running, the broker will check if there is some plugin configured and if was the broker will check for the methods implemented by the plugin.

In the broker startup, the broker will call tree methods: *mosquitto_auth_plugin_version*, *mosquitto_auth_plugin_init* and *mosquitto_auth_security_init*

### Mosquitto Basic Auth Workflow

The basic authentication API can be used only by implementing the method *mosquitto_auth_unpwd_check*.

If the *mosquitto_auth_unpwd_check* was implemented, each connection that come to the broker, the broker will call the method *mosquitto_auth_unpwd_check*.

If the method returns MOSQ_ERR_SUCCESS = 0 the broker will allow the client to connect. If returns anything > 0 the broker will deny the connection;

### Mosquitto ACL Workflow

For the Access Controller Layer, different for the basic auth, we will need to implement some methods and the broker will call each method following the diagram bellow:

<div align="center">
<img src="./docs/flow.png" />
</div>

### Requirements:
- Docker CE
- Rust LTS
```bash
asdf plugin-add rust https://github.com/code-lever/asdf-rust.git \
&& asdf install rust latest \
&& asdf global rust latest
```
- LLVM, CLang
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
- [Rust Mosquitto Plugin](https://github.com/TotalKrill/mosquitto_plugin)