#!/usr/bin/env bash

mkdir -p data
glslc resources/shader.vert -o data/shader.vert.spv
glslc resources/shader.frag -o data/shader.frag.spv
