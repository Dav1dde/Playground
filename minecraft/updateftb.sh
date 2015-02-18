#!/bin/bash

if [ "$#" -ne 2 ]; then
    echo "$0 <ftb installation> <server download>"
    exit 1
fi

src=$(readlink -e $1)
echo "[+] FTB installation: $src"

if [ ! -d "$src" ]; then
    echo "[!] No FTB installation found"
    exit 1
fi

backup=$src.$(date +"%d-%m-%Y")
echo "[+] Backup folder: $backup"

if [ -d "$backup" ]; then
    echo "[!] Backup folder already exists, remove or rename it first"
    exit 1
fi

set -e

tmp=$(mktemp -d)
echo "[+] Temporary folder: $tmp"
cd $tmp

echo "[+] Setting up new FTB"

echo "[-] Downloading FTB: $2"
wget --progress=bar:force -O mc.zip $2 2>&1 | tail -f -n +6
echo "[-] Unpacking"
unzip mc.zip >/dev/null

echo "[-] Removing leftover files"
rm mc.zip >/dev/null

echo "[-] Accepting eula"
sed -i -e s/false/true/ eula.txt

echo "[-] Installing minecraft files"
sh FTBInstall.sh 2>/dev/null >/dev/null

echo "[+] Copying old files"
rsync -a --ignore-existing --exclude '/config' --exclude '/libraries' --exclude '/mods' --exclude '/scripts' --exclude '/asm' $src/ $tmp/

echo "[+] Replacing old installation"
mv $src $backup
mv $tmp $src
