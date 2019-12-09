#!/bin/sh

cargo build
export PYMUT_PATH=$(readlink -e ./target/debug/pymut)

starting_directory=$(pwd)
tests_directory=./tests

for test in $tests_directory/*
do
    cd $test
    echo "Running $test"
    ./run.sh
    cd $starting_directory
done