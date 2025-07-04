#!/bin/bash

set -e

echo "🚧 Costruzione immagine Docker..."
docker build -f Dockerfile.musl -t agent-builder .

echo "📦 Estrazione file binari..."
mkdir -p output

# Usa un container temporaneo per copiare i file
docker create --name temp-agent agent-builder
docker cp temp-agent:/usr/local/bin/agent-test ./output/
docker cp temp-agent:/usr/local/bin/icon-cyber-green.png ./output/
docker cp temp-agent:/usr/local/bin/icon-cyber-red.png ./output/
docker rm temp-agent

echo "📁 Creazione struttura pacchetto .deb..."
rm -rf package
mkdir -p package/agent-test/usr/local/bin
mkdir -p package/agent-test/etc/systemd/system
mkdir -p package/agent-test/DEBIAN

# Copia binari e icone
cp output/agent-test package/agent-test/usr/local/bin/
cp output/icon-cyber-green.png package/agent-test/usr/local/bin/
cp output/icon-cyber-red.png package/agent-test/usr/local/bin/

# Crea file systemd
cat > package/agent-test/etc/systemd/system/agent-test.service <<EOF
[Unit]
Description=Agent Monitoraggio Rust
After=network.target

[Service]
ExecStart=/usr/local/bin/agent-test
Restart=always

[Install]
WantedBy=multi-user.target
EOF

# Crea file di controllo .deb
cat > package/agent-test/DEBIAN/control <<EOF
Package: agent-test
Version: 1.0.0
Section: utils
Priority: optional
Architecture: amd64
Maintainer: Giovanni D’Apote <email@example.com>
Description: Agent di monitoraggio con systemd service.
EOF

# Crea script post-install
cat > package/agent-test/DEBIAN/postinst <<EOF
#!/bin/bash
set -e

echo "🛠️  Abilito e avvio agent-test con systemd..."
systemctl enable agent-test || true
systemctl start agent-test || true
EOF

chmod +x package/agent-test/DEBIAN/postinst


echo "📦 Creo pacchetto .deb dentro container Debian..."
docker run --rm -v "$(pwd):/work" -w /work debian:bookworm \
    bash -c "apt-get update && apt-get install -y dpkg-dev && \
             dpkg-deb --build package/agent-test && \
             mv package/agent-test.deb output/agent-test_1.0.0_amd64.deb"

echo "✅ Pacchetto .deb creato: output/agent-test_1.0.0_amd64.deb"
