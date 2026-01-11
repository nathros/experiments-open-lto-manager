#!/bin/bash

GREEN="\e[0;32m"
RED="\e[0;31m"
RESET="\e[0m"

FAILURE=false
echo "Run through development environment check list:"

RUST_COMPILER="rustc"
if ! command -v $RUST_COMPILER >/dev/null 2>&1; then
    echo -e "Rust compiler    ($RUST_COMPILER) :$RED NOT FOUND$RESET"
    FAILURE=true
else
    echo -e "Rust compiler    ($RUST_COMPILER) :$GREEN FOUND$RESET"
fi

DIOXUS="dx"
if ! command -v $DIOXUS >/dev/null 2>&1; then
    echo -e "Dioxus toolchain ($DIOXUS)    :$RED NOT FOUND$RESET"
    FAILURE=true
else
    echo -e "Dioxus toolchain ($DIOXUS)    :$GREEN FOUND$RESET"
fi

if $FAILURE; then
    echo "Faults found"
    echo "See: https://dioxuslabs.com/learn/0.7/getting_started/"
else
    echo "No faults found"
fi