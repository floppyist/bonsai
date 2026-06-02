FROM voidlinux/voidlinux:latest

# FIX: Spiegelserver auf den offiziellen globalen HTTP-Mirror umstellen
RUN mkdir -p /etc/xbps.d && \
    echo "repository=http://repo-default.voidlinux.org/current" > /etc/xbps.d/00-repository-main.conf

# 1. System-Abhängigkeiten installieren (inkl. git, curl und tmux)
RUN xbps-install -Syu xbps && xbps-install -Syu && xbps-install -y \
    git \
    curl \
    tmux \
    cargo \
    llvm \
    clang \
    libclang \
    libopencv-devel \
    nodejs \
    pkg-config \
    cmake \
    make \
    gcc \
    zstd \
    libzstd \
    libzstd-devel \
    && rm -rf /var/cache/xbps/*

# Symlink für libclang (wichtig für die Rust-OpenCV-Bindings)
RUN LN_TARGET=$(ls /usr/lib/libclang.so.* | head -n 1) && ln -s $LN_TARGET /usr/lib/libclang.so

# 2. Repository direkt nach /app klonen
RUN git clone --branch feature/database https://github.com/floppyist/bonsai /app

# 3. Backend einmal vor-bauen (spart viel Zeit beim späteren Starten)
WORKDIR /app/backend
RUN cargo build --release

# 4. Frontend-Abhängigkeiten installieren
WORKDIR /app/frontend
RUN npm install

# 5. Ollama installieren & Modell direkt beim Image-Build herunterladen
RUN curl -fsSL https://ollama.com/install.sh | sh
# Trick: Ollama kurz im Hintergrund starten, Modell laden, wieder beenden
RUN ollama serve & \
    sleep 5 && \
    ollama pull qwen2.5vl:3b

# 6. Ein Start-Skript für Tmux generieren
RUN printf '#!/bin/sh\n\
# Tmux-Session erstellen und Ollama im ersten Fenster (Index 0) starten\n\
tmux new-session -d -s bonsai -n "ollama" "ollama serve"\n\
\n\
# Backend im zweiten Fenster (Index 1) starten (Nutzt das vor-gebaute Release)\n\
tmux new-window -t bonsai -n "backend" "cd /app/backend && cargo run --release"\n\
\n\
# Frontend im dritten Fenster (Index 2) starten\n\
tmux new-window -t bonsai -n "frontend" "cd /app/frontend && npm run dev -- --host"\n\
\n\
# Container aktiv halten, damit er nicht ausgeht\n\
tail -f /dev/null\n' > /entrypoint.sh && chmod +x /entrypoint.sh

# Ports freigeben (3000 = Backend, 5173 = Frontend)
EXPOSE 3000
EXPOSE 5173

WORKDIR /app
ENTRYPOINT ["/entrypoint.sh"]
