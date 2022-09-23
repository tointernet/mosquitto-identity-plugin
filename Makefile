install:
	@rm -rf ./.example/*.so
	@cargo build --release
	@cp ./target/release/libmosquitto_auth0_plugin.so ./.example/

docker: install
	@docker build -t ralvescosta/mosquitto-auth0 ./.example/
	@docker run -it -p 1883:1883 --platform linux/x86_64 ralvescosta/mosquitto-auth0:latest