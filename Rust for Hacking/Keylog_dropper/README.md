# Keyload dropper AKA Keylogger Dropper

### What is this ?

This is an dropper used to download keylogger and sender and exectute in background. 

### How does it work ?

When you execute the dropper, The keylogger and its sender will be dropped at Users Temp directory. 

Next, it will use windows API [CreateProcessW](https://learn.microsoft.com/en-us/windows/win32/api/processthreadsapi/nf-processthreadsapi-createprocessw) to execute the program as background.

It uses telegram bot to send the keycap.log(an file which stores keylog information) file for every 10 seconds. You can change the thread::sleep depend upon your needs 

### How to implement it ?

Just Clone these and compile both the programs. 

For key_exec: Change the URL.

For bot_send : Enter your telegram [BOT TOKEN](https://www.cytron.io/tutorial/how-to-create-a-telegram-bot-get-the-api-key-and-chat-id) and your [CHAT ID](https://www.alphr.com/find-chat-id-telegram/).

```
cargo build --release
```
keylogger.exe : An compiled version of [keylogger](https://github.com/Whitecat18/Rust-for-Malware-Development/tree/main/keylogger)

Host the file anywhere and execute the key_exec.exe

### Demos.

![exection](./pic/pic1.png)

Video: 

https://github.com/Whitecat18/Rust-for-Malware-Development/assets/96696929/d6fa54ff-0e7f-452f-a8a1-99e4259a6b8c





By 5mukx
