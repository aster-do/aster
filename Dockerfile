# =================================
# Step 1: Build backend
# =================================
FROM rust:bullseye AS build-backend

# Set cargo to use sparse registry
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse

WORKDIR /app
# Copy files to the container
COPY . .
# Build the project as release
RUN cargo build -p aster --release

# =================================
# Step 2: Build frontend
# =================================
FROM node:lts-bullseye AS build-frontend

WORKDIR /app
# Copy files to the container
COPY frontend/ .
# Install dependencies (npm ci makes sure the exact versions in the lockfile gets installed)
RUN npm ci 
# Build the app
RUN npm run build

# =================================
# Step 3: Run
# =================================
FROM debian:bullseye

ENV RUST_LOG=info
ENV ASTER_UI_PATH=/var/aster/ui

# Create a user to run the app
RUN useradd -u 1001 aster
USER aster

WORKDIR /etc/aster
# Copy the binary from the build container
COPY --from=build-backend /app/target/release/aster /etc/aster/aster
# Copy the frontend from the build container
COPY --from=build-frontend /app/build /var/aster/ui

# Run the binary
CMD ["./aster"]
