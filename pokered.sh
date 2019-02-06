#!/bin/bash

for (( flags = 0 ; flags < 4; flags = flags + 1 )); do
  pushd pokered
  make clean
  make ROMFLAGS=$flags
  cp pokered.gbc ../build/pokered$flags.gbc
  cp pokered.sym ../build/pokered$flags.sym
  popd
done

pushd pokered
make clean
popd
