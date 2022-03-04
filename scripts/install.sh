#!/bin/sh

sudo systemctl stop jackdntan.service
sudo systemctl disable jackdntan.service
cargo build --release
sudo mv ./target/release/jackdntan /var/jackdntan
sudo cp ./services/jackdntan.service ./jackdntan.service.backup
sudo mv ./services/jackdntan.service.backup /etc/systemd/system/jackdntan.service
sudo systemctl enable jackdntan.service
sudo systemctl start jackdntan.service