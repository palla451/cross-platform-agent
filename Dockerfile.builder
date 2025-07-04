FROM ubuntu:22.04

# Installa tool di build e librerie per Rust e .deb
RUN apt-get update && apt-get install -y \
    build-essential pkg-config curl \
    libgtk-3-dev libayatana-appindicator3-dev \
    libglib2.0-dev libgdk-pixbuf2.0-dev libpango1.0-dev \
    dpkg-dev dpkg \
    && rm -rf /var/lib/apt/lists/*

# Installa Rust
RUN curl https://sh.rustup.rs -sSf | bash -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Imposta la root del progetto
WORKDIR /app

# Copia tutto il progetto (agent-client, tray-icon, icons, debian, ecc.)
COPY . .

# 🔨 Compila agent-test
RUN cargo build --release

# 🔨 Compila agent-tray (nella cartella corretta)
WORKDIR /app/tray-icon
RUN cargo build --release

# 🔙 Torna alla root
WORKDIR /app

# 🧱 Costruisce la struttura del pacchetto .deb
RUN bash build-deb.sh

# 📦 Estrae il .deb finale
#CMD ["sh", "-c", "mkdir -p /output && cp -v package/*.deb /output/ && echo ✅ Pacchetto .deb copiato in /output"]
CMD ["sh", "-c", "mkdir -p /output && cp -v output/*.deb /output/ && echo ✅ Pacchetto .deb copiato in /output"]

