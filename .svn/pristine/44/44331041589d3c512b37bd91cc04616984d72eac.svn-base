FROM debian:bookworm

# Update default packages
RUN apt-get update

# PDF
RUN apt-get install -y texlive texinfo texlive-fonts-recommended texlive-fonts-extra texlive-latex-extra

RUN rm -rf /var/lib/apt/lists/*

CMD ["/bin/bash"]
