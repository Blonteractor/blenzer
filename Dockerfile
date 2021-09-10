FROM rust:alpine3.13

COPY . /bot

WORKDIR /bot

ENV DISCORD_TOKEN="ODgxNDYzNTE2MTQwMjg2MDAy.YStM3w.8UZF7yL-8iKwsXrFKn8BzkWekis"

RUN cargo build

CMD [ "cargo", "run" ]