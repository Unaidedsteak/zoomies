
# zoomies

Ever needed to click on a Zoom meeting invite and join the meeting on a remote host? No? Me either... :eyes::eyes:


:exclamation::exclamation::exclamation: **probably don't use this over the internet.** :exclamation::exclamation::exclamation:

  **How to cross-compile for Windows**
---
1. Install cross <br/>
	```cargo install cross``` 
2. Use cross to build Windows binaries <br/>
	```cross build --target x86_64-pc-windows-gnu```

  
  **Usage**
---

1. Place zoomies-client.exe on the host you'll be clicking meeting invites on.
2. Change a the following registry entry: <br/>
	```Computer\HKEY_CLASSES_ROOT\zoommtg\shell\open\command``` <br/>
	to the following command: <br/>
	```"C:\{{PATH_TO_ZOOMIES-SERVER.exe}}\zoomies-server.exe" "--url=%1"```
3. Place zoomies-server.exe on the host you want to join the meeting on.
4. Create a config.yml next to zoomies-server.exe (see example below)
5. Run zoomies-server.exe


**Example zoomies-server config**
---

```
zoom_bin_path:  "C:\\Users\\{{YOUR_USER}}\\AppData\\Roaming\\Zoom\\bin\\Zoom.exe"
```

