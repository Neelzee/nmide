FROM neelzee/tauri_img:latest

# Set up the working directory
WORKDIR /usr/src/app

# Copy the Tauri application code into the container
COPY . .

# Build the Tauri application
RUN yarn tauri build

CMD ["./src-tauri/target/release/nmide"]
