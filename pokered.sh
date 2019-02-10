#!/bin/bash

declare -a Roms=(\
  'pokered' 'pokeblue'\
  'pokered_girl' 'pokeblue_girl'\
  'pokered_items' 'pokeblue_items'\
  'pokered_girl_items' 'pokeblue_girl_items'\
)

for (( flags = 0 ; flags < ${#Roms[@]} ; flags = flags + 1 )); do
  pushd pokered
  make clean
  make ROMFLAGS=$flags
  rv=$?
  if [ $rv -ne 0 ]; then
  	exit $rv
  fi
  cp pokered.gb ../build/${Roms[$flags]}.gb
  cp pokered.sym ../build/${Roms[$flags]}.sym
  popd
done

pushd pokered
make clean
popd
