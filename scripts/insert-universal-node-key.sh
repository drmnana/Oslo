#!/bin/bash

if [ $# -ne 2 ]
  then
    echo "incorrect number of arguments"
    echo "command usage: ./insert-universal-node-key.sh secret-seed-in-quotes chain-spec-file-or-chain-type"
    echo "example usage: ./insert-universal-node-key.sh 'bottom drive obey lake curtain smoke basket hold race lonely fit walk' OsloMainnetChainSpec.json"
    exit 1
fi


echo "---NOTICE---"
echo "This script will take a single secret phrase enclosed in quotes and insert"
echo "it into the node as the Aura, Grandpa, and im_online private keys."
echo "It will also be used to derive and set the nodes PeerId. The previous"
echo "private keys and PeerId will be erased."
read -e -p "Do you want to continue? [y/N] : " insert


if [[ ("$insert" == "y" || "$insert" == "Y") ]];
  then
    keyjson=$(target/release/oslo-network key inspect --output-type json --scheme Ed25519 "$1")
    secretSeed=$(echo $keyjson | jq .secretSeed)
    secretSeed=${secretSeed:3:-1}
    if [[ $2 == "live" ]]; then
      nodekeydir=./node-storage/chains/Oslo-Network_Mainnet/network
    elif [[ $2 == "testnet" ]]; then
      nodekeydir=./node-storage/chains/Oslo-Network_Testnet/network
    elif [[ $2 =~ "Mainnet" ]]; then
      nodekeydir=./node-storage/chains/Oslo-Network_Mainnet/network
    elif [[ $2 =~ "Testnet" ]]; then
      nodekeydir=./node-storage/chains/Oslo-Network_Testnet/network
    else
      echo "Cannot determine node-key directory. Exiting..."
      exit 1 
    fi
    target/release/oslo-network key insert --base-path ./node-storage --suri "$1" --chain $2 --scheme Sr25519 --key-type imon
    target/release/oslo-network key insert --base-path ./node-storage --suri "$1" --chain $2 --scheme Sr25519 --key-type aura
    target/release/oslo-network key insert --base-path ./node-storage --suri "$1" --chain $2 --scheme Ed25519 --key-type gran
    mkdir -p $nodekeydir
    echo $secretSeed > $nodekeydir/secret_ed25519_hex
    xxd -r -p -c 0 $nodekeydir/secret_ed25519_hex $nodekeydir/secret_ed25519
    rm $nodekeydir/secret_ed25519_hex
    echo "Key inserted successfully"
fi