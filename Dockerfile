# Use the official image as a parent image.
FROM debian

# Set the working directory.
WORKDIR /app

# 可传参 docker build --build-arg RUST_ENV=alpha
ARG RUST_ENV=alpha
ENV RUST_ENV=$RUST_ENV
# ENV RUST_ENV=test
# Copy the file from your host to your current location.
# COPY package.json .

# Run the command inside your image filesystem.
# RUN npm install

# Inform Docker that the container is listening on the specified port at runtime.
# EXPOSE 8080

# Run the specified command within the container.
# CMD [ "/bin/bash" ]
CMD [ "/app/hello-world" ]

# Copy the rest of your app's source code from your host to your image filesystem.
COPY target/x86_64-unknown-linux-musl/debug/hello-world .