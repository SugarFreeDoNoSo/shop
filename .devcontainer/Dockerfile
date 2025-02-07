# Usa una imagen base de Rust (puedes elegir otra versión si lo requieres)
FROM rust:latest

# Instala dependencias del sistema, incluyendo las necesarias para Tauri y para que funcionen los file dialogs
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        wget \
        file \
        libxdo-dev \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libwebkit2gtk-4.0-dev \
        zenity \
        zsh \
        git \
        lld \
    && rm -rf /var/lib/apt/lists/*

# Instala NodeJS (requisito para Tauri, aquí se usa la versión 16 LTS)
RUN curl -fsSL https://deb.nodesource.com/setup_16.x | bash - && \
    apt-get install -y nodejs

# (Opcional) Instala bun si lo requieres para tu proyecto
RUN curl -fsSL https://bun.sh/install | bash

# Instala la toolchain nightly de Rust (si es necesaria)
RUN rustup toolchain install nightly

# Instala cargo-binstall para instalar binarios de forma eficiente
RUN cargo install cargo-binstall

# Instala dioxus-cli mediante cargo-binstall
RUN cargo binstall dioxus-cli

# (Opcional) Configuración de zsh para desarrollo:
# Agrega 'export DISPLAY=:0' para permitir la conexión X11 en WSL
RUN echo 'export DISPLAY=:0' >> /root/.bash

# Puedes continuar con otras configuraciones (como instalar oh-my-zsh o crear usuario no-root) según tu flujo de trabajo.
