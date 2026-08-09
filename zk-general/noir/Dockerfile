FROM ubuntu:24.04

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        curl \
        ca-certificates \
        bash \
        git && \
    rm -rf /var/lib/apt/lists/*

RUN curl -L https://raw.githubusercontent.com/noir-lang/noirup/main/install | bash

ENV PATH="/root/.nargo/bin:${PATH}"

RUN noirup

WORKDIR /app

COPY Nargo.toml ./
COPY src ./src
COPY Prover.toml ./

RUN nargo check

CMD ["nargo", "execute"]