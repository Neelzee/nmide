FROM debian:bookworm

# Update default packages
RUN apt-get update

# PDF
RUN apt-get install -y texlive
RUN apt-get install -y texinfo
RUN apt-get install -y texlive-fonts-recommended
RUN apt-get install -y texlive-fonts-extra
RUN apt-get install -y texlive-latex-extra

RUN rm -rf /var/lib/apt/lists/*

CMD ["/bin/bash"]
