#!/bin/bash
set -e

IMAGE_NAME="agent-full-builder"
OUTPUT_DIR="$(pwd)/output"

echo "🐳 Costruzione immagine Docker ($IMAGE_NAME)..."
docker build -f Dockerfile.builder -t "$IMAGE_NAME" .

echo "🚀 Esecuzione container per creare il pacchetto .deb..."
docker run --rm -v "$OUTPUT_DIR:/output" "$IMAGE_NAME"

echo "✅ Completato: pacchetto .deb disponibile in ./output/"
