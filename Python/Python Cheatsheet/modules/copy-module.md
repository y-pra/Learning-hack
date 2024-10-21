---
title: Python Copy Module - Python Cheatsheet
description: Copy Module is a set of functions that are related to copying different elements of a list, objects, arrays, etc. It can be used to create shallow copies as well as deep copies.
---

<base-title :title="frontmatter.title" :description="frontmatter.description">
Python Copy Module
</base-title>

Copy Module is a set of functions that are related to copying different elements of a list, objects, arrays, etc. It can be used to create shallow copies as well as deep copies.

<base-disclaimer>
  <base-disclaimer-title>
    From the Python 3 <a target="_blank" href="https://docs.python.org/3/library/copy.html">documentation</a>
  </base-disclaimer-title>
  <base-disclaimer-content>
    Assignment statements in Python do not copy objects, they create bindings between a target and an object. For collections that are mutable or contain mutable items, a copy is sometimes needed so one can change one copy without changing the other. This module provides generic <b>shallow</b> and <b>deep</b> copy operations.
  </base-disclaimer-content>
</base-disclaimer>

##  Shallow copy operations

Shallow copy constructs a new compound object and then (to the extent possible) inserts references into it to the objects found in the original.

copy.copy(x)
    Return a shallow copy of x.

```python
>>> import copy
>>> a = [[1],[2],[3]]
>>> b = copy.copy(a) ## this will copy the list a to list b

>>> a
#[[1], [2], [3]]
>>> b
#[[1], [2], [3]]
```
### Without importing copy module you can't use it

```python
# Traceback (most recent call last):
# File "<stdin>", line 1, in <module>
# NameError: name 'copy' is not defined
```
##  Deep copy operations

A deep copy constructs a new compound object and then, recursively, inserts copies into it of the objects found in the original.

copy.deepcopy(x[, memo])
  Return a deep copy of x.

```python
>>> import copy
>>> a = [[1],[2],[3]]
>>> b = copy.deepcopy(a) ## this will copy the list a to list b

>>> a[0][0] = 0
>>> a[1] = None

>>> a
#[[0], None, [3]]
>>> b
#[[1], [2], [3]]
```