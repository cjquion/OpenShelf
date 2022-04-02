#!/bin/bash

podman run -i --tty --name audio_demo --volume /dev/snd:/dev/snd audio_demo:latest
