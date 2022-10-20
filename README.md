# 🗑️ Rmt.rs

<p align="center">
    <img width="200" src="https://user-images.githubusercontent.com/53370597/195205359-21b93716-f78d-4200-9102-ce6145750303.png">
</p>
<p align="center"> <b>Fun fact</b>: <a href="https://stability.ai/blog/stable-diffusion-public-release">Stable diffusion</a> generated this logo 🎨</p>


**Rmt** is similar to the **rm** command, but it allows me to **save the deleted elements in the trash**. If you wish, you can restore the previously deleted elements of your choice (or delete them forever) with a **cli**.

![UmzJ1r8Z7D](https://user-images.githubusercontent.com/53370597/195318131-e1b3ad8b-4022-41c7-a226-3b9a28a1ee94.gif)

## ‼️ Rmt is not stable yet, do not use it for critical usages for the moment

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
rmt * -- -text.txt
rmt folder test.txt *.sh
```
✨ I like to use **-f** option, to remove all the warnings.

### 📺 Launch CLI to restore or flush elements

```sh
rmt --td
```

### ❌ Flush all element from the trash
```sh
rmt --tf
```

### 🔎 Informations about the trash

```sh
rmt --ti
```

## 🫵 Contribution

You can find all the information in the file [**CONTRIBUTING.md**](./CONTRIBUTING.md). Hoping to see you soon in my pull request 😊
