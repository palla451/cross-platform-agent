#!/bin/bash

# Abilita modalità strict
set -e

# Costanti
DOCKERFILE="Dockerfile.tray-builder"
IMAGE_NAME="tray-icon-builder"
OUTPUT_DIR="$(pwd)/output"

echo "🔨 Costruzione immagine Docker ($IMAGE_NAME)..."
docker build -f "$DOCKERFILE" -t "$IMAGE_NAME" .

echo "🚀 Esecuzione container per esportare il binario in $OUTPUT_DIR..."
docker run --rm -v "$OUTPUT_DIR:/output" "$IMAGE_NAME"

echo "✅ Completato: binario disponibile in ./output/agent-tray"
