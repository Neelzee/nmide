default:
  just --list

alias dev := run
alias bp := build-plugins

nmcdir := "nmide-core/src-tauri/"
nmlibc := "nmide-lib/"
thesis := "nmide-thesis"
docker_user := "neelzee"

run:
  npm run tauri dev

pdf:
  pdflatex -shell-escape --output-directory={{thesis}} {{thesis}}/main.tex

# Builds Docker Images
docker-build:
  docker build -f nmide-docker/tauri.Dockerfile . -t nmide-tauri:latest # Tauri
  docker build -f nmide-docker/full.Dockerfile . -t nmide-full:latest # Full
  docker build -f nmide-docker/thesis.Dockerfile . -t nmide-thesis:latest # Thesis
  docker build -f nmide-docker/rust.Dockerfile . -t nmide-rust:latest # Rust Testing
  docker build -f nmide-docker/node.Dockerfile . -t nmide-node:latest # Node Testing
  docker build -f nmide-docker/c.Dockerfile . -t nmide-c:latest # C Testing
  

# Tags Docker Images for release
docker-tag:
  docker tag nmide-tauri:latest {{docker_user}}/nmide-tauri:latest
  docker tag nmide-full:latest {{docker_user}}/nmide-full:latest
  docker tag nmide-thesis:latest {{docker_user}}/nmide-thesis:latest
  docker tag nmide-rust:latest {{docker_user}}/nmide-rust:latest
  docker tag nmide-node:latest {{docker_user}}/nmide-node:latest
  docker tag nmide-c:latest {{docker_user}}/nmide-c:latest

# Publishes Docker Images
docker-push:
  docker push {{docker_user}}/nmide-tauri:latest
  docker push {{docker_user}}/nmide-full:latest
  docker push {{docker_user}}/nmide-thesis:latest
  docker push {{docker_user}}/nmide-rust:latest
  docker push {{docker_user}}/nmide-node:latest
  docker push {{docker_user}}/nmide-c:latest

docker-full:
  just docker-build
  just docker-tag
  just docker-push

svn:
  git clean -x
  svn add . --force
  svn commit -m "Push changes to SVN Repo" --username ${SVN_USERNAME} --password ${SVN_PASSWORD} --non-interactive

# Initialized C Dev. Env.
make-init:
  -cd {{nmlibc}} && git clone https://github.com/nemequ/munit.git
  @cd {{nmlibc}}release && pwd
  cd {{nmlibc}}release && cmake -DCMAKE_BUILD_TYPE=Release ..

# Removes all unused C files
make-clean:
  rm -rf {{nmlibc}}build
  rm -rf {{nmlibc}}release
  rm -rf {{nmlibc}}debug
  rm -rf {{nmlibc}}munit

# Creates release
make-release:
  mkdir -p {{nmlibc}}release
  @cd {{nmlibc}}release && pwd
  cd {{nmlibc}}release && cmake -DCMAKE_BUILD_TYPE=Release ..
  cd {{nmlibc}}release && make

# Runs c-lib tests
make-test:
  cd {{nmlibc}}debug && make && ./test_cmap
  cd {{nmlibc}}debug && make && ./test_cmodel

# Runs leak checks
make-check:
  cd {{nmlibc}} && cppcheck --enable=all --force --quiet -imunit -ibuild -idebug -irelease .
  cd {{nmlibc}}debug && valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ./test_cmodel || true
  cd {{nmlibc}}debug && valgrind --leak-check=full --show-leak-kinds=all --track-origins=yes --verbose ./test_cmap || true
