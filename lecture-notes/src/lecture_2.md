# Lecture 2 - File Systems

## `umask` Redux

The `umask` (user file creation mask) command in Unix and Unix-like operating systems is used to set default permissions for newly created files and directories. It dictates how the default permissions are masked (restricted) for these newly created files and directories.

### Understanding `umask`

Permissions are represented by a three-digit octal number:

- `0` means read, write, and execute permissions are granted.
- `1` means execute permission is not granted.
- `2` means write permission is not granted.
- `4` means read permission is not granted.

The values are additive. For example, `2` (write) and `4` (read) combine to make `6` (read and write).

### Default Permissions

Typically, the default permissions for new files are `0666` (read and write for everyone), and for new directories, `0777` (read, write, and execute for everyone).

### Example Usage

To see the current `umask` value:

```sh
umask
```

Setting a new `umask` value, for example, `027`:

```sh
umask 027
```

This command would result in new files being created with permissions `0640` (666 - 027) and new directories with permissions `0750` (777 - 027).

### Common umask Values

- `002`: Files are created with `664` and directories with `775` (group can write).
- `022`: Files are created with `644` and directories with `755` (group can't write).
- `027`: Files are created with `640` and directories with `750` (more restrictive for the group).

### Practical Example

If you execute `umask 002`, the newly created files will have read and write permissions for the owner and group, but only read permission for others:

```sh
touch newfile
ls -l newfile
# -rw-rw-r-- 1 user group 0 date time newfile
```

In summary, `umask` is an essential command for controlling default file permissions in a Unix system, ensuring that new files and directories are created with permissions that do not allow more access than desired.

## Groups

## Assignment 1: Six Degrees of Kevin Bacon
