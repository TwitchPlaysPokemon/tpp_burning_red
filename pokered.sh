#!/bin/bash

NUM_ROMS=8

for (( flags = 0 ; flags < NUM_ROMS ; flags = flags + 1 )); do
  pushd pokered
  make clean
  make ROMFLAGS=$flags
  rv=$?
  if [ $rv -ne 0 ]; then
  	exit $rv
  fi
  cp pokered.gb ../build/pokered$flags.gb
  cp pokered.sym ../build/pokered$flags.sym
  popd
done

pushd pokered
make clean
popd
