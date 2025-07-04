#!/bin/bash
set -e

NAME="agent-client"
VERSION="1.0.0"
ARCH="amd64"
BUILD_DIR="package/${NAME}_${VERSION}_${ARCH}"

echo "🧹 Pulizia vecchi build..."
rm -rf "$BUILD_DIR"

echo "📁 Creo struttura pacchetto..."
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/local/bin"
mkdir -p "$BUILD_DIR/usr/share/icons/hicolor/48x48/apps"
mkdir -p "$BUILD_DIR/etc/systemd/system"
mkdir -p "$BUILD_DIR/etc/agent"
cp .env "$BUILD_DIR/etc/agent/.env"


echo "🔨 Compilo il binario principale (agent-client)..."
cargo build --release

echo "📦 Copio binari..."
cp target/release/agent-client "$BUILD_DIR/usr/local/bin/"

# Usa il binario precompilato della tray da output/
#if [ ! -f output/agent-tray ]; then
#  echo "❌ ERRORE: agent-tray non trovato in output/. Compilalo con build.tray-icon.sh prima di eseguire questo script."
#  exit 1
#fi
#cp output/agent-tray "$BUILD_DIR/usr/local/bin/"
#cp tray-icon/target/release/agent-tray "$BUILD_DIR/usr/local/bin/"
cp ./tray-icon/target/release/agent-tray "$BUILD_DIR/usr/local/bin/"



echo "🎨 Copio icone..."
cp icons/icon-cyber-green.png "$BUILD_DIR/usr/share/icons/hicolor/48x48/apps/"
cp icons/icon-cyber-red.png "$BUILD_DIR/usr/share/icons/hicolor/48x48/apps/"

echo "⚙️ Copio file systemd..."
cp debian/agent_client.service "$BUILD_DIR/etc/systemd/system/"
cp debian/agent_tray.service "$BUILD_DIR/etc/systemd/system/"

echo "📝 Scrivo DEBIAN/control..."
cat > "$BUILD_DIR/DEBIAN/control" <<EOF
Package: $NAME
Version: $VERSION
Architecture: $ARCH
Maintainer: Giovanni D'Apote
Description: Agent endpoint con tray icon integrata
EOF

echo "⚙️ Creo script postinst..."
cat > "$BUILD_DIR/DEBIAN/postinst" <<'EOF'
#!/bin/bash
set -e

chmod +x /usr/local/bin/agent-client
chmod +x /usr/local/bin/agent-tray

systemctl daemon-reexec
systemctl daemon-reload

systemctl enable agent_client.service
systemctl start agent_client.service || true

systemctl enable agent_tray.service
systemctl start agent_tray.service || true

gtk-update-icon-cache /usr/share/icons/hicolor || true
EOF

chmod +x "$BUILD_DIR/DEBIAN/postinst"

echo "📦 Creo pacchetto .deb..."
dpkg-deb --build "$BUILD_DIR" "output/${NAME}_${VERSION}_${ARCH}.deb"


echo "✅ Fatto! Pacchetto creato: ${BUILD_DIR}.deb"
