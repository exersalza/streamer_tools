#!/bin/bash


while getopts p: flag
do 
  case "${flag}" in 
    p) prod=${OPTARG};;
  esac
done

export PROD="false"
tailwind=""
trunk=""

if [[ $prod -eq 1 ]]; then
  export PROD="true"
  tailwind="--minify"
  trunk="--release --public-url /"
fi


cargo watch -w . -s "tailwind -o ./css/tailwind.css $tailwind && trunk build $trunk"
