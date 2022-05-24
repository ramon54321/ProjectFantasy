#!/usr/bin/env bash

mkdir -p data
glslc resources/shader.frag -o data/frag.spv
