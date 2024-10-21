---
title: Python Pathlib Module - Python Cheatsheet
description: This module provides a portable way of using operating system dependent functionality.
---

<base-title :title="frontmatter.title" :description="frontmatter.description">
Python Os Module
</base-title>

This module provides a portable way of using operating system dependent functionality.

<base-disclaimer>
  <base-disclaimer-title>
    Pathlib vs Os Module
  </base-disclaimer-title>
  <base-disclaimer-content>
    The <router-link to="/modules/pathlib-module">pathlib</router-link> module provides a lot more functionality than the ones listed here, like getting file name, getting file extension, reading/writing a file without manually opening it, etc. See the <a target="_blank" href="https://docs.python.org/3/library/pathlib.html">official documentation</a> if you intend to know more.
  </base-disclaimer-content>
</base-disclaimer>

## Linux and Windows Paths

On Windows, paths are written using backslashes (`\`) as the separator between
folder names. On Unix based operating system such as macOS, Linux, and BSDs,
the forward slash (`/`) is used as the path separator. Joining paths can be
a headache if your code needs to work on different platforms.

Fortunately, Python provides us with `os.path.join` to handle this.

```python
>>> import os

>>> os.path.join('usr', 'bin', 'spam')
# 'usr\\bin\\spam'
```

Joining paths is helpful if you need to create different file paths under
the same directory.

```python
>>> my_files = ['accounts.txt', 'details.csv', 'invite.docx']

>>> for filename in my_files:
...     print(os.path.join('C:\\Users\\asweigart', filename))
...
# C:\Users\docs\accounts.txt
# C:\Users\docs\details.csv
# C:\Users\docs\invite.docx
```

## The current working directory

```python
>>> import os

>>> os.getcwd()
# 'C:\\Python34'
>>> os.chdir('C:\\Windows\\System32')

>>> os.getcwd()
# 'C:\\Windows\\System32'
```

## Creating new folders

```python
>>> import os
>>> os.makedirs('C:\\delicious\\walnut\\waffles')
```

## Absolute vs. Relative paths

There are two ways to specify a file path.

- An **absolute path**, which always begins with the root folder
- A **relative path**, which is relative to the program’s current working directory

There are also the dot (`.`) and dot-dot (`..`) folders. These are not real folders, but special names that can be used in a path. A single period (“dot”) for a folder name is shorthand for “this directory.” Two periods (“dot-dot”) means “the parent folder.”

### Handling Absolute paths

To see if a path is an absolute path:

```python
>>> import os
>>> os.path.isabs('/')
# True

>>> os.path.isabs('..')
# False
```

You can also extract an absolute path:

```python
>>> import os
>>> os.getcwd()
'/home/docs'

>>> os.path.abspath('..')
'/home'
```

### Handling Relative paths

You can get a relative path from a starting path to another path.

```python
>>> import os
>>> os.path.relpath('/etc/passwd', '/')
# 'etc/passwd'
```

## Path and File validity

### Checking if a file/directory exists

```python
>>> import os

>>> os.path.exists('.')
# True

>>> os.path.exists('setup.py')
# True

>>> os.path.exists('/etc')
# True

>>> os.path.exists('nonexistentfile')
# False
```

### Checking if a path is a file

```python
>>> import os

>>> os.path.isfile('setup.py')
# True

>>> os.path.isfile('/home')
# False

>>> os.path.isfile('nonexistentfile')
# False
```

### Checking if a path is a directory

```python
>>> import os

>>> os.path.isdir('/')
# True

>>> os.path.isdir('setup.py')
# False

>>> os.path.isdir('/spam')
# False
```

## Getting a file's size in bytes

```python
>>> import os

>>> os.path.getsize('C:\\Windows\\System32\\calc.exe')
# 776192
```

## Listing directories

```python
>>> import os

>>> os.listdir('C:\\Windows\\System32')
# ['0409', '12520437.cpx', '12520850.cpx', '5U877.ax', 'aaclient.dll',
# --snip--
# 'xwtpdui.dll', 'xwtpw32.dll', 'zh-CN', 'zh-HK', 'zh-TW', 'zipfldr.dll']
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
>>> import os
>>> total_size = 0

>>> for filename in os.listdir('C:\\Windows\\System32'):
...     total_size = total_size + os.path.getsize(os.path.join('C:\\Windows\\System32', filename))
...
>>> print(total_size)
# 1117846456
```

## Deleting files and folders

- Calling `os.unlink(path)` will delete the file at path.

- Calling `os.rmdir(path)` will delete the folder at path. This folder must be empty of any files or folders.

## Walking a Directory Tree

```python
>>> import os
>>>
>>> for folder_name, subfolders, filenames in os.walk('C:\\delicious'):
...     print(f'The current folder is {folder_name}')
...     for subfolder in subfolders:
...         print(f'SUBFOLDER OF {folder_name}: {subfolder}')
...     for filename in filenames:
...         print(f'FILE INSIDE {folder_name}: filename{filename}')
...     print('')
...
# The current folder is C:\delicious
# SUBFOLDER OF C:\delicious: cats
# SUBFOLDER OF C:\delicious: walnut
# FILE INSIDE C:\delicious: spam.txt

# The current folder is C:\delicious\cats
# FILE INSIDE C:\delicious\cats: catnames.txt
# FILE INSIDE C:\delicious\cats: zophie.jpg

# The current folder is C:\delicious\walnut
# SUBFOLDER OF C:\delicious\walnut: waffles

# The current folder is C:\delicious\walnut\waffles
# FILE INSIDE C:\delicious\walnut\waffles: butter.txt
```
