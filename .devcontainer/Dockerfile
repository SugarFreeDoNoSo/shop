FROM rust:1.84.1-bullseye

RUN apt-get update && \
    apt-get install -y --no-install-recommends \
        build-essential \
        curl \
        wget \
        file \
        libxdo-dev \
        libssl-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev \
        libwebkit2gtk-4.0-dev \
        zsh \
        git \
        software-properties-common # Needed for add-apt-repository

# Install cargo-binstall
RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash

# Install dioxus-cli
RUN cargo binstall dioxus-cli

# Set zsh as default shell
SHELL ["/bin/zsh", "-c"]

# Install oh-my-zsh
RUN sh -c "$(curl -fsSL https://raw.github.com/ohmyzsh/ohmyzsh/master/tools/install.sh)"

# Install powerlevel10k
RUN git clone --depth=1 https://github.com/romkatv/powerlevel10k.git ${ZSH_CUSTOM:-$HOME/.oh-my-zsh/custom}/themes/powerlevel10k

# Configure zsh to use powerlevel10k.  This assumes you want the "classic" prompt style.  Adjust as desired.
RUN echo 'POWERLEVEL9K_LEFT_PROMPT_ELEMENTS=(dir vcs git status)' >> ~/.zshrc
RUN echo 'POWERLEVEL9K_RIGHT_PROMPT_ELEMENTS=()' >> ~/.zshrc # Or whatever you want on the right
RUN echo 'POWERLEVEL9K_MODE=classic' >> ~/.zshrc
RUN echo 'ZSH_THEME="powerlevel10k/powerlevel10k"' >> ~/.zshrc

# Source zshrc to apply changes (important!)
RUN source ~/.zshrc
