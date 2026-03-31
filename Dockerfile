# compiling on my system with release doesn't work on other systems 
# bcoz arch btw
# this is a dockerfile that does let me compile my project
# such that it works on ubuntu
# and through wsl on windows
FROM ubuntu:20.04
ENV DEBIAN_FRONTEND=noninteractive
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    cmake \
    libx11-dev \
    libxcursor-dev \
    libxrandr-dev \
    libxi-dev \
    libgl1-mesa-dev \
    libxinerama-dev \
    clang \
    libclang-dev \
    && rm -rf /var/lib/apt/lists/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
WORKDIR /app
CMD ["/bin/bash"]
