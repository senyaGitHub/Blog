+++
date = 2023-08-16
weight = 1
title = "systemd timers"
+++

[Systemd timers](https://wiki.archlinux.org/title/systemd/Timers) are a feature of the systemd init system on Linux that allows to schedule and automate tasks at specific intervals or times.

## Query and View Timer Information


- View the active timers `systemctl list-timers` 
- View the timer unit file contents `systemctl cat fstrim.timer`
- View the associated service unit file contents `systemctl cat fstrim.service`
- Analyse various timer notations: 
```
	systemd-analyze calendar hourly
	systemd-analyze calendar daily
	systemd-analyze calendar weekly
	systemd-analyze calendar monthly
	systemd-analyze calendar "*-*-* 09..17:00/5"
	systemd-analyze calendar "Mon..Fri *-*-* 09..17:00/5"
```
# Creating Timer event
- Create a timer unit file `sudo systemctl edit --full --force [NAME].timer`
-  Edit the file.
- Create an associated service unit file `sudo systemctl edit --full --force [NAME].service`
- Edit the file as well.
- Create a script to be called by the service `vim [NAME.sh]`
- Enable and start the timer `sudo systemctl enable [NAME].timer` and  `sudo systemctl start blahwoof.timer`
- Troubleshoot any issues. Look in the system journal to see if everything is OK ``journalctl --follow``
- Make script executable `chmod a+x ~/journo.sh`

# Updates by Timer for Debian
- Create service `sudo vim /etc/systemd/system/update.service`
- The this code inside 
	```
	[Unit] Description=Automated OS Update 
	[Service] ExecStart=/usr/bin/apt-get update && /usr/bin/apt-get dist-upgrade -y 
	Type=oneshot
	[Install] WantedBy=multi-user.target
	```
- Create timer and reference service `sudo vim /etc/systemd/system/update.timer`
	```
	[Unit]
	 Description=Runs the update service every 5 days 
	[Timer]
	OnBootSec=1min OnUnitActiveSec=5d 
	Unit=update.service 
	[Install] 
	 WantedBy=timers.target
	 ```
- Enable and start `sudo systemctl enable update.timer %% systemctl start update.timer`
