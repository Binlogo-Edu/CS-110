# Lecture 3 - Layering, Naming, and Filesystem Design

## Memory

- just like RAM, hard drives provide us with **a contiguous stretch of memory** where we can store data.
- information in RAM is **byte-addressable**, meaning that each byte has a unique address.
- similarly, information on a hard drive is **sector-addressable**, must read or write an entire sector at a time.
- sectors are often **512 bytes** in size, but not always.

## Readwrite

- API might look like this:

```c++
class Drive {
public:
  size_t getSectorSize() const;
  void readSector(size_t num, unsigned char data[]) const;
  void writeSector(size_t num, const unsigned char data[]);
};
```

## Block vs Sector

- Sectors are the smallest unit of storage on a hard drive, they are the physical storage units on the hard drive.
- Blocks are the smallest unit of storage that the filesystem can address and operate.

## Filesystem

file metadata

- **boot block**: contains the bootstrap code that the computer executes when it boots up.
- **superblock**: contains metadata about the filesystem.
- **inodes**: includes the size of the filesystem, the size of the blocks, and the number of blocks in the filesystem.

file contents

- file payloads are stored in **data blocks**.

## The inode

- In Unix, the inode is a data structure that stores information about a file to track which blocks are associated with that file.
- *inodes* are 32-byte data structures that contain metadata about files.
  - stored within an inode: file size, permissions, timestamps, and pointers to the blocks that store the file's data.

```c
struct inode {
  uint16_6 i_mode; // file type and permissions
  uint8_t i_nlink; // number of hard links to the file
  uint8_t i_uid; // user ID of the file's owner
  uint8_t i_gid; // group ID of the file's group
  uint8_t i_size0; // most significant bits of the file size
  uint16_t i_size1; // least significant bits of the file size
  uint16  uint16_t i_addr[8]; // block numbers for the file's data
  uint16_t i_atime[2]; // last access time
  uint16_t i_mtime[2]; // last modification time
};
```
