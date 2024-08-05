# Lecture 1 - Introduction

- Instructors:
  - [Chris Gregg]
  - [Nick Troccoli]

Thanks to them for creating such an amazing course, and making it available to the public! 🫶

## Overview

7 big principles:

- Abstraction
- Modularity and Layering
- Naming and Name Resolution <- **Important and Hard 🐕**
- Caching
- Virtualization
- Concurrency
- Client-Server request-response <- **What the Internet is built on**

### Abstraction seperates behavior from implementation

Think about a file list result of the linux command `ls -l`:

- how are files stored on the computer?
- if everything is a 0 or a 1 to a computer, there must be some translation and abstraction going on.
- there're infinite ways to store files, but the abstraction of a file system is a way to organize them.

Things an operating system designer has to think about to design a file system

- How are the files stored? (HHD or SSD), what's the actual low-level form of the file storage.
  > [!IMPORTANT]
  > The files must be located when required.
- What's the relationship between a file's name and its data location? <- **Hard part to abstraction**
- Are small files stored diffirently than large files? <- **Performance**
- How are files deleted? Make sure no space going to waste.
- Can two filenames point to the same file? To avoid space waste.
- Does file data share the same space as file metadata? <- (think: Heap Allocator...)

**No matter how those questions are answered, the user should not be able to tell the difference.**

### Modularity and Layering

Add the `-i` flag to the `ls` command to see the inode number of the file.

### Naming and Name Resolution

what the inode is that is associated with that particular path.

## Syllabus

### Overview of Linux Filesystems

- Linux and C libraries for file manipulation: `stat`, `struct stat`, `open`, `close`,`read`, `write`, `readdir`, `struct dirent`, file descriptors, regular files, directories, soft and hard links, programmatic manipulation of them, implementation of `ls`, `cp`, `find`, and other core Unix utilities you probably never realized were plain old C
programs
- Naming, abstraction and layering concepts in systems as a means for managing complexity, blocks, inodes, inode pointer structure, inode as abstraction over blocks, direct blocks, indirect blocks, doubly indirect blocks, design and implementation of a file system
- Multiprocessing and Exceptional Control Flow
  - Introduction to multiprocessing, fork, waitpid, execvp, process ids, interprocess communication, context switches, user versus kernel mode, system calls and how their calling convention differs from those of normal functions
  - Protected address spaces, virtual memory, virtual to physical address mapping, scheduling
Concurrency versus parallelism, multiple cores versus multiple processors, concurrency issues with multiprocessing, signal masks

### Pros and cons of file descriptors over FILE pointers and C++ iostreams

- The fd abstraction provides direct, low lovel access to stream of data without the fuss of data structures or objects. It certainly can't be slower.
- FILE pointers and C++ iostreams work well when you know you're interacting with standard output, standard input, and local files.
  - They are less useful when the stream of bytes is associated with a network connection.
  - FILE pointers and C++ iostreams assume they can rewind and move the file pointer back and forth freely, but that's not the case with file descriptors associated with network connections.
- File descriptors, however, work with read and write and little else used in this course.
- C FILE pointers and C++ streams, on the other hand, provide automatic buffering and more elaborate formatting options.
