---
title: Python Pathlib Module - Python Cheatsheet
description: The pathlib module was added in Python 3.4, offering an object-oriented way to handle file system paths.
---

<base-title :title="frontmatter.title" :description="frontmatter.description">
Python Pathlib Module
</base-title>

The `pathlib` module was added in Python 3.4, offering an object-oriented way to handle file system paths.

<base-disclaimer>
  <base-disclaimer-title>
    Pathlib vs Os Module
  </base-disclaimer-title>
  <base-disclaimer-content>
    <code>pathlib</code> provides a lot more functionality than the ones from <code>os</code> and listed here, like getting file name, getting file extension, reading/writing a file without manually opening it, etc. See the <a target="_blank" href="https://docs.python.org/3/library/pathlib.html">official documentation</a> if you intend to know more.
  </base-disclaimer-content>
</base-disclaimer>

## Linux and Windows Paths

On Windows, paths are written using backslashes (`\`) as the separator between
folder names. On Unix based operating system such as macOS, Linux, and BSDs,
the forward slash (`/`) is used as the path separator. Joining paths can be
a headache if your code needs to work on different platforms.

Fortunately, Python provides us with `pathlib.Path.joinpath` to easily handle this.

```python
>>> from pathlib import Path

>>> print(Path('usr').joinpath('bin').joinpath('spam'))
# usr/bin/spam
```

`pathlib` also provides a shortcut to joinpath using the `/` operator:

```python
>>> from pathlib import Path

>>> print(Path('usr') / 'bin' / 'spam')
# usr/bin/spam
```

Joining paths is helpful if you need to create different file paths under
the same directory.

```python
>>> my_files = ['accounts.txt', 'details.csv', 'invite.docx']
>>> home = Path.home()
>>> for filename in my_files:
...     print(home / filename)
...
# /home/docs/accounts.txt
# /home/docs/details.csv
# /home/docs/invite.docx
```

## The current working directory

```python
>>> from pathlib import Path
>>> from os import chdir

>>> print(Path.cwd())
# /home/docs

>>> chdir('/usr/lib/python3.10')
>>> print(Path.cwd())
# /usr/lib/python3.10
```

## Creating new folders

```python
>>> from pathlib import Path
>>> cwd = Path.cwd()
>>> (cwd / 'delicious' / 'walnut' / 'waffles').mkdir()
# Traceback (most recent call last):
#   File "<stdin>", line 1, in <module>
#   File "/usr/lib/python3.10/pathlib.py", line 1226, in mkdir
#     self._accessor.mkdir(self, mode)
#   File "/usr/lib/python3.10/pathlib.py", line 387, in wrapped
#     return strfunc(str(pathobj), *args)
# FileNotFoundError: [Errno 2] No such file or directory: '/home/docs/delicious/walnut/waffles'
```

The reason of this error is that the 'delicious' directory does
not exist, so we cannot make the 'walnut' and the 'waffles' directories under
it. To fix this, do:

```python
>>> from pathlib import Path
>>> cwd = Path.cwd()
>>> (cwd / 'delicious' / 'walnut' / 'waffles').mkdir(parents=True)
```

## Absolute vs. Relative paths

There are two ways to specify a file path.

- An **absolute path**, which always begins with the root folder
- A **relative path**, which is relative to the program’s current working directory

There are also the dot (`.`) and dot-dot (`..`) folders. These are not real folders, but special names that can be used in a path. A single period (“dot”) for a folder name is shorthand for “this directory.” Two periods (“dot-dot”) means “the parent folder.”

### Handling Absolute paths

To see if a path is an absolute path:

```python
>>> from pathlib import Path
>>> Path('/').is_absolute()
# True

>>> Path('..').is_absolute()
# False
```

You can also extract an absolute path:

```python
>>> from pathlib import Path
>>> print(Path.cwd())
# /home/docs

>>> print(Path('..').resolve())
# /home
```

### Handling Relative paths

You can get a relative path from a starting path to another path.

```python
>>> from pathlib import Path
>>> print(Path('/etc/passwd').relative_to('/'))
# etc/passwd
```

## Path and File validity

### Checking if a file/directory exists

```python
from pathlib import Path

>>> Path('.').exists()
# True

>>> Path('setup.py').exists()
# True

>>> Path('/etc').exists()
# True

>>> Path('nonexistentfile').exists()
# False
```

### Checking if a path is a file

```python
>>> from pathlib import Path

>>> Path('setup.py').is_file()
# True

>>> Path('/home').is_file()
# False

>>> Path('nonexistentfile').is_file()
# False
```

### Checking if a path is a directory

```python
>>> from pathlib import Path

>>> Path('/').is_dir()
# True

>>> Path('setup.py').is_dir()
# False

>>> Path('/spam').is_dir()
# False
```

## Getting a file's size in bytes

```python
>>> from pathlib import Path

>>> stat = Path('/bin/python3.10').stat()
>>> print(stat) # stat contains some other information about the file as well
# os.stat_result(st_mode=33261, st_ino=141087, st_dev=2051, st_nlink=2, st_uid=0,
# --snip--
# st_gid=0, st_size=10024, st_atime=1517725562, st_mtime=1515119809, st_ctime=1517261276)

>>> print(stat.st_size) # size in bytes
# 10024
```

## Listing directories

```python
>>> from pathlib import Path

>>> for f in Path('/usr/bin').iterdir():
...     print(f)
...
# ...
# /usr/bin/tiff2rgba
# /usr/bin/iconv
# /usr/bin/ldd
# /usr/bin/cache_restore
# /usr/bin/udiskie
# /usr/bin/unix2dos
# /usr/bin/t1reencode
# /usr/bin/epstopdf
# /usr/bin/idle3
# ...
```

## Directory file sizes

<base-warning>
  <base-warning-title>
    WARNING
  </base-warning-title>
  <base-warning-content>
    Directories themselves also have a size! So, you might want to check for whether a path is a file or directory using the methods in the methods discussed in the above section.
  </base-warning-content>
</base-warning>

```python
>>> from pathlib import Path

>>> total_size = 0
>>> for sub_path in Path('/usr/bin').iterdir():
...     total_size += sub_path.stat().st_size
...
>>> print(total_size)
# 1903178911
```

## Deleting files and folders

- Calling `Path.unlink()` will delete the file at path.

- Calling `Path.rmdir()` will delete the folder at path. This folder must be empty of any files or folders.
