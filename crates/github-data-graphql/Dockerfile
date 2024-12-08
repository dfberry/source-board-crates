# docker build --no-cache -t ghcli:v1 .
# docker run -p 4000:4000 ghcli:v1 --org MicrosoftDocs --repo node-essentials --pat your_personal_access_token
FROM rust AS builder

WORKDIR /app

# Copy the actual source code
COPY . .

# Build the dependencies
RUN cargo build --release

# Expose port 4000
EXPOSE 4000

ENV PORT=4000

CMD ["/bin/bash"]