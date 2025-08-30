#!/bin/bash
pushd .
cd server
bash ./buildModules.sh
popd
bash ./startServer.sh
