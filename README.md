# 🗑️ Rmt.rs

**Rmt** is similar to the **rm** command, but it allows me to **save the deleted elements in the trash**. If you wish, you can restore the previously deleted elements of your choice (or delete them forever) with a **cli**.
![1K3V18y64t](https://user-images.githubusercontent.com/53370597/194941244-875fe124-dc7a-4fba-ab39-2caba739f2eb.gif)



## 👨🏽‍💻 Installation



⚡️ Quick start

Download the binary depending on your configuration here: https://github.com/AmineZouitine/rmt.rs/realeases

Then you just need to enter this command in your terminal:
```sh
tar -xf <downloaded_archive> cpad && sudo mv cpad /usr/local/bin
````

## Features

### 🚮 Delete an element (but it is saved in the trash don't worry)

```sh
rmt [OPTIONS] [FILES|FOLDER]

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
