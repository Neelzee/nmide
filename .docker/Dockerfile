# Use the base image
FROM neelzee/tauri_img:latest

# Copy the Tauri source code to the container
COPY . .

# Install dependencies
RUN yarn install

# Build the Tauri application
RUN yarn tauri build

# Entrypoint command to run the built Tauri application
ENTRYPOINT [ "./src-tauri/target/release/nmide" ] 
