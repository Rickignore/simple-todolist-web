FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates --no-install-recommends \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy binary ELF
COPY web_todolist /app/web_todolist

# Copy folder static HTML/CSS/JS
COPY static /app/static

# Izinkan binary dijalankan
RUN chmod +x /app/web_todolist

# Railway atau Render menggunakan port dari ENV
ENV PORT=8080

EXPOSE 8080

CMD ["./web_todolist"]