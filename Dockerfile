FROM alpine:3.14

# All subsequent instructions will start from this directory.
# WORKDIR /app

# =============================================================================

# Install prerequisites for Rust download.
RUN apk add ca-certificates     # HTTPS connections
RUN apk add curl 

# Install prerequisites for Rust install.
RUN apk add gcc                 # C compiler

# Install Rust toolchain (nightly).
RUN curl --proto '=https' \
         --tlsv1.2 -sSf https://sh.rustup.rs | \
         sh -s -- --default-toolchain nightly -y

# Add Rust toolchain to $PATH.
ENV PATH=$PATH:/root/.cargo/bin

RUN apk add --no-cache musl-dev
RUN apk add --no-cache libc-dev

# =============================================================================

# Install `make` tool
RUN apk add make
#
# RUN apk add build-base

# Copy this directory (Rust Code) into Docker Image
COPY . .

# # Set environment variable for port.
# # ENV PORT=8080
# # Make app listen on port 8080, so we can access NodeJS app publicly.
# # EXPOSE 8080

# =============================================================================

# CMD ["echo", "\naaaaa\n"]
CMD ["make"]
