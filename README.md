# groucho

![A disembodied nose wearing a pair of glasses like Groucho Marx](resources/icon-groucho.png)

![An application for generating AI images using Stable Diffusion](resources/screenshot.png)

A frontend for Huggingface's `candle` library, written in GTK for Linux desktop clients.

## Getting Started

`nix run .` will compile and run the program locally. `nix profile install .` will install the app to your system with a desktop entry hooked through Nix. 

## How it works

Everything is local. Generating your first image will automatically download the selected Stable Diffusion version to the `~/.cache/huggingface/hub/` folder. 

## Building

`nix develop .` will create a development shell for the project. 

Non-Nix systems will need the GTK depedencies. 