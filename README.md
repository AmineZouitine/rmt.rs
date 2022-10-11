# 🗑️ Rmt.rs

<p align="center">
    <img width="200" src="https://user-images.githubusercontent.com/53370597/195205359-21b93716-f78d-4200-9102-ce6145750303.png">
</p>
<p align="center"> <b>Fun fact</b>: <a href="https://stability.ai/blog/stable-diffusion-public-release">Stable diffusion</a> generated this logo 🎨</p>


**Rmt** is similar to the **rm** command, but it allows me to **save the deleted elements in the trash**. If you wish, you can restore the previously deleted elements of your choice (or delete them forever) with a **cli**.


![PG0c5IFyWI](https://user-images.githubusercontent.com/53370597/195192037-c5c557b2-e8bb-42c7-beb6-9dbf03f9ff71.gif)

## ‼️ Rmt.rs is not yet stable, do not use it for critical applications yet

## 👨🏽‍💻 Installation



### ⚡️ Quick start

Download the binary depending on your configuration here: https://github.com/AmineZouitine/rmt.rs/releases

Then you just need to enter this command in your terminal:
```sh
tar -xf <downloaded_archive> rmt && sudo mv rmt /usr/local/bin
````

### 😎 Pro tip (optional)

Add **rmt** as an **alias** instead of the **rm** command.
## Features

### 🚮 Delete an element (but it is saved in the trash don't worry)

```sh
rmt [OPTION]... [FILE|FOLDER]...

Exemples: 
rmt text.txt
rmt *
rmt folder test.txt *.sh
```
✨ I like to use **-f** option, to remove all the warnings.

### 📺 Launch CLI to restore or flush elements

```sh
rmt trash_display
or
rmt td
```

### ❌ Flush all element from the trash
```sh
rmt trash_flush
or
rmt tf
```

### 🔎 Informations about the trash

```sh
rmt trash_info
or
rmt ti
```

## 🫵 Contribution

You can find all the information in the file **CONTRIBUTING.md**. Hoping to see you soon in my pull request 😊
