FROM ivangabriele/tauri:debian-bookworm-18

RUN apt-get install -y jq

# PDF Generation
RUN apt-get install -y texlive
RUN apt-get install -y texinfo
RUN apt-get install -y texlive-fonts-recommended
RUN apt-get install -y texlive-fonts-extra
RUN apt-get install -y texlive-latex-extra

# Rust-Nmide-FFI
RUN apt-get install -y libclang-dev

# Ease of project building
RUN cargo install just

CMD ["/bin/bash"]
