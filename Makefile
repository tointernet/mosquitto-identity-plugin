install:
	@rm -rf ./.example/*.so
	@cargo build --release
	@cp ./target/release/libidentity_plugin.so ./.example/

docker: install
	@docker build -t ralvescosta/mosquitto-identity ./.example/
	@docker run -it -p 1883:1883 --platform linux/x86_64 ralvescosta/mosquitto-identity:latest