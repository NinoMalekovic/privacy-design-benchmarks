FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        git \
        nodejs \
        npm \
        build-essential && \
    rm -rf /var/lib/apt/lists/*

# Install Circom
RUN curl -L https://github.com/iden3/circom/releases/download/v2.2.2/circom-linux-amd64 \
    -o /usr/local/bin/circom && \
    chmod +x /usr/local/bin/circom

WORKDIR /app

# Give npm a real project directory.
RUN npm init -y && \
    npm install --no-audit --no-fund circomlib

COPY . .

CMD ["bash"]