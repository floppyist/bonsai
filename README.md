<table border="0" cellpadding="0" cellspacing="0" style="border: none; border-collapse: collapse;">
  <tr style="border: none;">
    <td style="border: none; padding-right: 10px;"><img src="frontend/public/icon.gif" alt="Bonsai Logo" width="45"></td>
    <td style="border: none; padding: 0;"><h1 style="border: none; margin: 0; padding: 0;"><strong>bonsai</strong></h1></td>
  </tr>
</table>

> Keep your pantry and purchases perfectly trimmed.

Bonsai is a smart Progressive Web App (PWA) designed to automate your grocery tracking and budgeting. By simply scanning your receipts and EAN barcodes, the app automatically populates your digital pantry—complete with real-time stock levels and nutritional insights—while seamlessly managing your household expense log in the background.

---

![Axum](https://img.shields.io/badge/Axum-orange?style=for-the-badge&logo=rust&logoColor=white)
![Tokio](https://img.shields.io/badge/Tokio-red?style=for-the-badge&logo=tokio&logoColor=white)
![Vue.js](https://img.shields.io/badge/vuejs-%234FC08D.svg?style=for-the-badge&logo=vuedotjs&logoColor=white)
![OpenCV](https://img.shields.io/badge/opencv-%235C3EE8.svg?style=for-the-badge&logo=opencv&logoColor=white)
![Ollama](https://img.shields.io/badge/Ollama-black?style=for-the-badge&logo=ollama&logoColor=white)

---

<!--toc:start-->
- [Installation (Void Linux)](#installation-void-linux)
  - [System Dependencies](#system-dependencies)
  - [Ollama Setup](#ollama-setup)
- [Running the Application](#running-the-application)
  - [Backend Server](#backend-server)
  - [Frontend Client](#frontend-client)
<!--toc:end-->

## Installation (Void Linux)

### System Dependencies

Install the core compilers, runtimes, and C++ bindings required for the image processing pipeline:

```bash
sudo xbps-install -y cargo llvm libclang libopencv-devel nodejs
```

> [!NOTE]  
> Void Linux holds multiple versions of `libclang`, but our Rust bindings explicitly look for `libclang.so`. You need to create a proper symlink.

1. **Identify your installed version:**
   ```bash
   ls /lib | grep libclang
   # Example output:
   # libclang-cpp.so.21.1
   # libclang.so.21.1
   # libclang.so.21.1.7
   ```

2. **Create the symlink** (adjust the version string matching your output):
   ```bash
   cd /lib
   sudo ln -s libclang.so.21.1.7 libclang.so
   ```

### Ollama Setup

Bonsai utilizes local Vision-Language Models (VLM) for receipt extraction.

1. Download and install [Ollama](https://ollama.com/).
2. Start the background service:
   ```bash
   ollama serve
   ```
3. Pull and run the optimized vision model:
   ```bash
   ollama run qwen2.5vl:3b
   ```

---

## Running the Application

### Backend Server

Navigate to the backend directory and spin up the native Axum/Tokio-powered Rust engine:

```bash
cd backend
cargo run --release
```

### Frontend Client

Install the web dependencies and launch the Vue/Vite development server mapped to your local network:

```bash
cd frontend
npm install
npm run dev -- --host
```
