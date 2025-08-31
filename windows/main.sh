#!/bin/bash

OS_NAME=$(uname -s)

if [[ "$OS_NAME" == *"MINGW"* ]] || [[ "$OS_NAME" == *"CYGWIN"* ]] || [[ "$OS_NAME" == *"MSYS"* ]]; then
    echo "Get a VM and try to get this on Linux, mainly Ubuntu or Debian."
    exit 1
elif [[ "$OS_NAME" == "Linux" ]]; then
    echo "Linux detected. Continuing..."
else
    echo "Unsupported OS. Try Linux, mainly Ubuntu or Debian."
    exit 1
fi
