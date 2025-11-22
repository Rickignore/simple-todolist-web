# ---------- build stage ----------
FROM rust:1.76 as builder
WORKDIR /usr/src/app

# cache deps
COPY Cargo.toml Cargo.lock ./
# create dummy src to cache cargo downloads
RUN mkdir src && echo "fn main() { println!(\"dummy\"); }" > src/main.rs
RUN cargo build --release || true

# copy actual source
COPY . .
RUN rm -f src/main.rs
RUN cargo build --release

# ---------- runtime stage ----------
FROM debian:bookworm-slim
# install ca-certificates (network calls) and tzdata if needed
RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends \
 && rm -rf /var/lib/apt/lists/*

# create non-root user
RUN useradd -m appuser
WORKDIR /home/appuser

# copy binary + static files
COPY --from=builder /usr/src/app/target/release/todo_rust_js /usr/local/bin/todo_rust_js
COPY --from=builder /usr/src/app/static ./static

# set permissions
RUN chown -R appuser:appuser /home/appuser
USER appuser

# Railway gives port in $PORT — default to 8080 if not set by env.
ENV PORT=8080

# expose (informational)
EXPOSE 8080

# Use shell form to allow env expansion if you ever reference $PORT in command (note: our binary reads $PORT).
CMD ["./web_todolist"]