#!/bin/bash

PROJECT=fineval
tmux new-session -d -s $PROJECT
tmux new-window -n code
tmux new-window -n compile
