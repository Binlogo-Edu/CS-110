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
