FROM ubuntu:latest

COPY /* /etc/mosquitto/conf.d/
RUN apt-get update -y && apt install mosquitto -y
EXPOSE 1883

CMD [ "mosquitto", "-c", "/etc/mosquitto/conf.d/default.conf" ]