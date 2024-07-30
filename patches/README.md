# OpenSSH Versions
You can use either OpenSSH-9.3-janky or OpenSSH-9.8. OpenSSH-9.3-janky is guaranteed to work but has an issue where it gets stuck at "activating" on startup, though it still allows SSH access to the console. On the other hand, OpenSSH-9.8 does not have this issue, has been correctly patched, and has minimal modifications compared to OpenSSH-9.3-janky. Starting from OpenSSH-9.7, a penalty system similar to ipban was introduced, which may need to be disabled to prevent banning users who attempt to brute force their way into your server. For details on disabling this feature, refer to the initial readme file.

# Building Pam
```
cmake .
make
```