<div align="center">
  <img src="frontend/public/icon.gif" alt="Bonsai Logo" width="96">

# Bonsai

**Keep your pantry and purchases perfectly trimmed.**

Smart receipt scanning, barcode tracking, pantry management, and budgeting — all running locally.



<div style="display: flex; height: 100px; gap: 10px; width: 100%; margin-top: 20px; margin-bottom: 20px;">
  <img style="flex: 1;" src="docs/screenshots/dashboard.png" width="220">
  <img style="flex: 1;" src="docs/screenshots/scan.png" width="220">
  <img style="flex: 1;" src="docs/screenshots/receipts.png" width="220">
</div>

<p align="center">
  Dashboard • Scanner • Receipts
</p>

<p>
  <img src="https://img.shields.io/badge/Axum-orange?style=for-the-badge&logo=rust&logoColor=white" alt="Axum">
  <img src="https://img.shields.io/badge/Tokio-red?style=for-the-badge&logo=tokio&logoColor=white" alt="Tokio">
  <img src="https://img.shields.io/badge/Vue.js-4FC08D?style=for-the-badge&logo=vuedotjs&logoColor=white" alt="Vue.js">
  <img src="https://img.shields.io/badge/OpenCV-5C3EE8?style=for-the-badge&logo=opencv&logoColor=white" alt="OpenCV">
  <img src="https://img.shields.io/badge/Ollama-black?style=for-the-badge&logo=ollama&logoColor=white" alt="Ollama">
</p>

</div>

---

Bonsai is a privacy-friendly, open source Progressive Web App (PWA) and `axum` backend for managing groceries, pantry stock, and household expenses.

By scanning receipts and EAN barcodes, Bonsai automatically builds and maintains a digital pantry, tracks inventory levels, enriches products with nutritional information, and keeps an expense log in the background.

Everything runs locally and can be self-hosted.

---

> [!CAUTION]
> **Work in Progress (WIP)**
>
> This project is currently in a very early development stage. Features might change drastically, break unexpectedly, or remain incomplete. Use with caution!

---

## Features

* 📄 Receipt scanning using local Vision Language Models (VLMs)
* 📦 Automatic pantry management
* 🔍 EAN barcode scanning and lookup
* 🥗 Nutritional information support
* 💰 Household expense tracking
* 📱 Installable Progressive Web App (PWA)
* 🔒 Self-hosted and privacy-friendly
* 🐳 Docker deployment support

---

## Table of Contents

* [Installation](#installation)

  * [System Dependencies](#system-dependencies)
  * [Backend Setup](#backend-setup)
  * [Optional: Import OpenFoodFacts Data](#optional-import-openfoodfacts-data)
  * [Ollama Setup](#ollama-setup)
  * [Docker Deployment](#docker-deployment)
* [Running Bonsai](#running-bonsai)

  * [Backend](#backend)
  * [Frontend](#frontend)
* [Architecture](#architecture)

---

# Installation

## System Dependencies

### Void Linux

Install the required compilers, libraries, and runtime dependencies:

```bash
sudo xbps-install -y \
    cargo \
    llvm \
    libclang \
    libopencv-devel \
    nodejs
```

> [!NOTE]
> The Rust OpenCV bindings expect `libclang.so` to exist. On Void Linux you may need to create a symbolic link manually.

List available libclang files:

```bash
ls /lib | grep libclang
```

Example output:

```text
libclang-cpp.so.21.1
libclang.so.21.1
libclang.so.21.1.7
```

Create the required symlink:

```bash
cd /lib
sudo ln -s libclang.so.21.1.7 libclang.so
```

Adjust the version number to match your installed system.

---

## Backend Setup

Create the default directory structure:

```bash
mkdir -p backend/db
mkdir -p backend/certs
```

### Generate Development Certificates

Navigate to the certificates directory:

```bash
cd backend/certs
```

Create a local certificate authority:

```bash
openssl genrsa -out devCA.key 4096

openssl req -x509 -new -nodes \
    -key devCA.key \
    -sha256 \
    -days 3650 \
    -subj "/CN=Dev-Root-CA/O=Development/C=DE" \
    -addext "basicConstraints=critical,CA:TRUE" \
    -addext "keyUsage=digitalSignature,keyCertSign,cRLSign" \
    -out devCA.pem
```

Create the server key and certificate request:

```bash
openssl genrsa -out server.key 2048

openssl req -new \
    -key server.key \
    -subj "/CN=bonsai.pi/O=Development/C=DE" \
    -out server.csr
```

Create the extension file:

```bash
touch server.ext
```

Insert the following content:

```ini
authorityKeyIdentifier=keyid,issuer
basicConstraints=CA:FALSE
keyUsage=digitalSignature, nonRepudiation, keyEncipherment, dataEncipherment
extendedKeyUsage=serverAuth
subjectAltName=@alt_names

[alt_names]
DNS.1=localhost
DNS.2=your.domain
IP.1=127.0.0.1
IP.2=192.168.0.42
```

Generate the server certificate:

```bash
openssl x509 -req \
    -in server.csr \
    -CA devCA.pem \
    -CAkey devCA.key \
    -CAcreateserial \
    -out server.crt \
    -days 397 \
    -sha256 \
    -extfile server.ext
```

---

## Optional: Import OpenFoodFacts Data

Generate the default databases:

```bash
cd backend
cargo run
```

Place your OpenFoodFacts SQLite dump into:

```text
backend/db/off.db
```

Connect to the EAN database:

```bash
sqlite3 backend/db/ean.db
```

Import product data:

```sql
ATTACH DATABASE 'off.db' AS off;

INSERT INTO ean_products (
    ean,
    name,
    brand,
    category,
    nutrients
)
SELECT
    ean,
    name_de,
    brand,
    'UNCATEGORIZED',
    nutrients
FROM off.off_products;

DETACH DATABASE off;
```

---

## Ollama Setup

Bonsai uses local Vision Language Models (VLMs) for receipt extraction.

Install Ollama and start the service:

```bash
ollama serve
```

Download the recommended model:

```bash
ollama run qwen2.5vl:3b
```

> [!TIP]
> The model only needs to be downloaded once.

---

## Docker Deployment

If you prefer not to install OpenCV, Rust, Node.js, and Ollama manually, the complete stack can be deployed using Docker.

```bash
cd docker
docker compose up --build
```

---

# Running Bonsai

## Backend

Start the Axum backend:

```bash
cd backend
cargo run --release
```

---

## Frontend

Install dependencies and launch the development server:

```bash
cd frontend
npm install
npm run dev -- --host
```

The application will be available on your local network.

---

# Architecture

```text
┌─────────────────┐
│   Vue.js PWA    │
└────────┬────────┘
         │ HTTPS
         ▼
┌─────────────────┐
│   Axum Backend  │
└───────┬─────────┘
        │
  ┌─────┴─────┐
  ▼           ▼
SQLite      Ollama
(EAN DB)    (VLM OCR)
```

---

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
