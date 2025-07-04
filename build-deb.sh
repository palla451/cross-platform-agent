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
mkdir -p "$BUILD_DIR/usr/share/agent"
mkdir -p "$BUILD_DIR/etc/systemd/system"
mkdir -p "$BUILD_DIR/etc/agent"
mkdir -p "$BUILD_DIR/etc/skel/.config/autostart"

echo "📄 Copio .env..."
cp .env "$BUILD_DIR/etc/agent/.env"

echo "🔨 Compilo il binario principale (agent-client)..."
cargo build --release

echo "📦 Copio binari..."
cp target/release/agent-client "$BUILD_DIR/usr/local/bin/"
cp tray-icon/target/release/agent-tray "$BUILD_DIR/usr/local/bin/"

echo "🎨 Copio icone..."
cp icons/icon-cyber-green.png "$BUILD_DIR/usr/share/icons/hicolor/48x48/apps/"
cp icons/icon-cyber-red.png "$BUILD_DIR/usr/share/icons/hicolor/48x48/apps/"

echo "⚙️ Copio file systemd..."
cp debian/agent_client.service "$BUILD_DIR/etc/systemd/system/"

echo "🖥️ Copio file .desktop in /etc/skel/.config/autostart/..."
cp debian/agent-tray.desktop "$BUILD_DIR/etc/skel/.config/autostart/"

echo "🖥️ Copio anche in /usr/share/agent per il postinst..."
cp debian/agent-tray.desktop "$BUILD_DIR/usr/share/agent/"

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

gtk-update-icon-cache /usr/share/icons/hicolor || true

# 💡 AUTOSTART agent-tray per l'utente attivo
GUI_USER=$(who | grep '(:0)' | head -n1 | awk '{print $1}')
USER_HOME=$(eval echo "~$GUI_USER")

if [ -n "$GUI_USER" ] && [ -d "$USER_HOME" ]; then
  echo "📎 Copio agent-tray.desktop per l'utente: $GUI_USER"
  mkdir -p "$USER_HOME/.config/autostart"
  cp /usr/share/agent/agent-tray.desktop "$USER_HOME/.config/autostart/"
  chown "$GUI_USER:$GUI_USER" "$USER_HOME/.config/autostart/agent-tray.desktop"
else
  echo "⚠️ Nessun utente grafico attivo trovato"
fi
EOF

chmod +x "$BUILD_DIR/DEBIAN/postinst"

echo "📦 Creo pacchetto .deb..."
dpkg-deb --build "$BUILD_DIR" "output/${NAME}_${VERSION}_${ARCH}.deb"

echo "✅ Fatto! Pacchetto creato: output/${NAME}_${VERSION}_${ARCH}.deb"
