let container_id = docker run --detach ubuntu:latest | split row "\n" | first
