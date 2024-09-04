FROM neelzee/nmide-rust:latest

RUN apt-get update && apt-get install cmake -y

CMD ["/bin/bash"]
