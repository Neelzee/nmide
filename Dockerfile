FROM ivangabriele/tauri:debian-bookworm-18

RUN apt-get install -y jq
RUN apt-get install -y texlive
RUN apt-get install -y texinfo
RUN apt-get install -y texlive-fonts-recommended
RUN apt-get install -y texlive-fonts-extra
RUN apt-get install -y texlive-latex-extra
RUN cargo install just

CMD ["/bin/bash"]
