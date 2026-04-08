#!/bin/bash

SESSION="bridge_session"

# Убиваем старую сессию если есть
tmux kill-session -t $SESSION 2>/dev/null

# Создаём новую сессию
tmux new-session -d -s $SESSION -n main

# Панель 0 (левая верхняя) — anvil
tmux send-keys -t $SESSION:0.0 "anvil" C-m

# Делим вертикально (создаём правую панель)
tmux split-window -h -t $SESSION:0

# Панель 1 (правая верхняя) — solana validator + anchor
tmux send-keys -t $SESSION:0.1 "cd bridge && solana-test-validator --geyser-plugin-config geyser-config.json" C-m

# Делим левую панель горизонтально (нижняя левая)
tmux select-pane -t $SESSION:0.0
tmux split-window -v -t $SESSION:0.0

# Панель 2 (левая нижняя) — app
tmux send-keys -t $SESSION:0.2 "cd app/bridge_app && npm install && npm run start" C-m

# Делим правую панель горизонтально (нижняя правая)
tmux select-pane -t $SESSION:0.1
tmux split-window -v -t $SESSION:0.1

# Панель 3 (правая нижняя) — solidity
tmux send-keys -t $SESSION:0.3 "cd solidity && npx hardhat run scripts/deploy.js --network localhost && npx hardhat run scripts/simulation.js --network localhost" C-m

# Делаем красивый layout
tmux select-layout -t $SESSION tiled

# Подключаемся
tmux attach -t $SESSION