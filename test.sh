#!/bin/bash

SESSION="bridge_session"

# Убиваем старую сессию если есть
tmux kill-session -t $SESSION 2>/dev/null

# Создаем новую сессию
tmux new-session -d -s $SESSION


tmux rename-window -t $SESSION "anvil"
tmux send-keys -t $SESSION "anvil" C-m


tmux new-window -t $SESSION -n "anchor"
tmux send-keys -t $SESSION "cd bridge" C-m
tmux solana-test-validator  --geyser-plugin-config geyser-config.json 
tmux send-keys -t $SESSION "anchor test" C-m


tmux new-window -t $SESSION -n "app"
tmux send-keys -t $SESSION "cd app/bridge_app" C-m
tmux send-keys -t $SESSION "npm install" C-m
tmux send-keys -t $SESSION "npm run start" C-m


tmux new-window -t $SESSION -n "solidity"
tmux send-keys -t $SESSION "cd solidity" C-m
tmux send-keys -t $SESSION "npx hardhat run scripts/deploy.js --network localhost" C-m
tmux send-keys -t $SESSION "npx hardhat run scripts/simulation.js --network localhost" C-m


tmux attach-session -t $SESSION