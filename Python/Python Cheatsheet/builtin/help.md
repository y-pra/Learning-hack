---
title: Python help() built-in function - Python Cheatsheet
description: Invoke the built-in help system. (This function is intended for interactive use.) If no argument is given, the interactive help system starts on the interpreter console. If the argument is a string, then the string is looked up as the name of a module, function, class, method, keyword, or documentation topic, and a help page is printed on the console.
---

<base-title :title="frontmatter.title" :description="frontmatter.description">
Python help() built-in function
</base-title>

<base-disclaimer>
  <base-disclaimer-title>
    From the <a target="_blank" href="https://docs.python.org/3/library/functions.html#help">Python 3 documentation</a>
  </base-disclaimer-title>
  <base-disclaimer-content>
   Invoke the built-in help system. (This function is intended for interactive use.) If no argument is given, the interactive help system starts on the interpreter console. If the argument is a string, then the string is looked up as the name of a module, function, class, method, keyword, or documentation topic, and a help page is printed on the console.
  </base-disclaimer-content>
</base-disclaimer>

## Examples

```python
>>> help(type)
# Help on class type in module builtins:

# class type(object)
#  |  type(object_or_name, bases, dict)
#  |  type(object) -> the object's type
#  |  type(name, bases, dict) -> a new type
#  |
#  |  Methods defined here:
#  |
#  |  __call__(self, /, *args, **kwargs)
#  |      Call self as a function.
#  |
#  |  __delattr__(self, name, /)
#  |      Implement delattr(self, name).
#  |
#  |  __dir__(self, /)
#  |      Specialized __dir__ implementation for types.
#  |
#  |  __getattribute__(self, name, /)
# :

>>> help(str)
# Help on class str in module builtins:

# class str(object)
#  |  str(object='') -> str
#  |  str(bytes_or_buffer[, encoding[, errors]]) -> str
#  |
#  |  Create a new string object from the given object. If encoding or
#  |  errors is specified, then the object must expose a data buffer
#  |  that will be decoded using the given encoding and error handler.
#  |  Otherwise, returns the result of object.__str__() (if defined)
#  |  or repr(object).
#  |  encoding defaults to sys.getdefaultencoding().
#  |  errors defaults to 'strict'.
#  |
#  |  Methods defined here:
# :

>>> help(help)
# Help on _Helper in module _sitebuiltins object:

# class _Helper(builtins.object)
#  |  Define the builtin 'help'.
#  |
#  |  This is a wrapper around pydoc.help that provides a helpful message
#  |  when 'help' is typed at the Python interactive prompt.
#  |
#  |  Calling help() at the Python prompt starts an interactive help session.
#  |  Calling help(thing) prints help for the python object 'thing'.
#  |
#  |  Methods defined here:
#  |
#  |  __call__(self, *args, **kwds)
#  |      Call self as a function.
#  |
# :
```
