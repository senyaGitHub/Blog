+++
title = "	Notes on Using Systemctl"
date = 2017-09-23
weight = 1
+++

## Notes on Using Systemctl 
Systemctl is an important utility for managing systemd-based systems. It is needed to unify service configurations and behaviour across Linux distros. ts primary component is a "system and service manager" – an init system used to bootstrap user space and manage user processes. It also provides replacements for various daemons and utilities, including device management, login management, network connection management, and event logging
## Viewing Systemd Environment and Service Status 
- Use `systemctl show --property=[PROPERTY]` to view a specific property of a service.
- Use `systemctl status [SERVICE]` to view the status of a service. 
- Use `systemctl list-units` to list all units loaded in the system.
## Installing and Managing Services with Systemctl
- Use `apt install [systemD PACKAGE]` to install a package. 
- Use `systemctl start [SERVICE]` to start a service. 
- Use `systemctl stop [SERVICE]` to stop a service. 

## Changing Service Parameters with Systemctl 
- Use `systemctl edit [SERVICE]` to create/edit a drop-in unit configuration file. 
- Use `systemctl daemon-reload` to reload daemon configuration files. 
- Use `systemctl restart [SERVICE]` to restart a service after changes have been made. 

## Reverting a Service Parameter Change 
- Stop the service with `systemctl stop [SERVICE]`. 
- Remove the config file with `rm -rf (carefully) /etc/system.control/[SERVICE]`. 
- Restart the service with `systemctl deamon-reload` and  `systemctl restart`.

## Example troubleshooting `target.service`
- Check what is wrong `systemctl status target.service -l`.
- Use preferred editor to edit the issue in the config file, vim in my case `vim /etc/systemd/system/target.service`.
- Fix the issue.
- Reload the service `systemctl daemon-reload` and `systemctl start target.service`
- Check if the issues resolved. `systemctl status target.service`
- Enable the service (in order for service to start when booting)`systemctl enable target.service` and `reboot`.
