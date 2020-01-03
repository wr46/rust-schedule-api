FROM rustlang/rust:nightly

WORKDIR /app
COPY . .
EXPOSE 8000
RUN cargo build
#RUN cargo build --release
ENV ROCKET_ADDRESS="0.0.0.0"
CMD ["cargo", "run"]
