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
