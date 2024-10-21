---
title: Python Virtual environments - Python Cheatsheet
description: The use of a Virtual Environment is to test python code in encapsulated environments and to also avoid filling the base Python installation with libraries we might use for only one project.
---

<base-title :title="frontmatter.title" :description="frontmatter.description">
Virtual Environment
</base-title>

The use of a Virtual Environment is to test python code in encapsulated environments, and to also avoid filling the base Python installation with libraries we might use for only one project.

## virtualenv

1.  Install virtualenv

        pip install virtualenv

1.  Install virtualenvwrapper-win (Windows)

        pip install virtualenvwrapper-win

Usage:

1.  Make a Virtual Environment named `HelloWorld`

        mkvirtualenv HelloWorld

    Anything we install now will be specific to this project. And available to the projects we connect to this environment.

1.  Set Project Directory

    To bind our virtualenv with our current working directory we simply enter:

        setprojectdir .

1.  Deactivate

    To move onto something else in the command line type `deactivate` to deactivate your environment.

        deactivate

    Notice how the parenthesis disappear.

1.  Workon

    Open up the command prompt and type `workon HelloWorld` to activate the environment and move into your root project folder

        workon HelloWorld

## Poetry

<base-disclaimer>
  <base-disclaimer-title>
    From <a href="https://python-poetry.org/">Poetry website</a>
  </base-disclaimer-title>
  <base-disclaimer-content>
    Poetry is a tool for dependency management and packaging in Python. It allows you to declare the libraries your project depends on and it will manage (install/update) them for you.
  </base-disclaimer-content>
</base-disclaimer>

1.  Install Poetry

        pip install --user poetry

2.  Create a new project

        poetry new my-project

    This will create a my-project directory:

        my-project
        ├── pyproject.toml
        ├── README.rst
        ├── poetry_demo
        │   └── __init__.py
        └── tests
            ├── __init__.py
            └── test_poetry_demo.py

    The pyproject.toml file will orchestrate your project and its dependencies:

        [tool.poetry]
        name = "my-project"
        version = "0.1.0"
        description = ""
        authors = ["your name <your@mail.com>"]

        [tool.poetry.dependencies]
        python = "*"

        [tool.poetry.dev-dependencies]
        pytest = "^3.4"

3.  Packages

    To add dependencies to your project, you can specify them in the tool.poetry.dependencies section:

        [tool.poetry.dependencies]
        pendulum = "^1.4"

    Also, instead of modifying the pyproject.toml file by hand, you can use the add command and it will automatically find a suitable version constraint.

        $ poetry add pendulum

    To install the dependencies listed in the pyproject.toml:

        poetry install

    To remove dependencies:

        poetry remove pendulum

For more information, check the [documentation](https://poetry.eustace.io/docs/) or read here:

- <router-link to="/blog/python-projects-with-poetry-and-vscode-part-1">Python projects with Poetry and VSCode. Part 1</router-link>
- <router-link to="/blog/python-projects-with-poetry-and-vscode-part-2">Python projects with Poetry and VSCode. Part 2</router-link>
- <router-link to="/blog/python-projects-with-poetry-and-vscode-part-3">Python projects with Poetry and VSCode. Part 3</router-link>

## Pipenv

<base-disclaimer>
  <base-disclaimer-title>
    From <a target="_blank" href="https://pipenv.pypa.io/en/latest/">Pipenv website</a>
  </base-disclaimer-title>
  <base-disclaimer-content>
    Pipenv is a tool that aims to bring the best of all packaging worlds (bundler, composer, npm, cargo, yarn, etc.) to the Python world. Windows is a first-class citizen, in our world.
  </base-disclaimer-content>
</base-disclaimer>

1.  Install pipenv

        pip install pipenv

2.  Enter your Project directory and install the Packages for your project

        cd my_project
        pipenv install <package>

    Pipenv will install your package and create a Pipfile for you in your project’s directory. The Pipfile is used to track which dependencies your project needs in case you need to re-install them.

3.  Uninstall Packages

        pipenv uninstall <package>

4.  Activate the Virtual Environment associated with your Python project

        pipenv shell

5.  Exit the Virtual Environment

        exit

Find more information and a video in [docs.pipenv.org](https://docs.pipenv.org/).

## Anaconda

<base-disclaimer>
  <base-disclaimer-title>
    <a target="k" href="https://anaconda.com/">Anaconda</a> is another popular tool to manage python packages.
  </base-disclaimer-title>
  <base-disclaimer-content>
    Where packages, notebooks, projects and environments are shared. Your place for free public conda package hosting.
  </base-disclaimer-content>
</base-disclaimer>

Usage:

1.  Make a Virtual Environment

        conda create -n HelloWorld

2.  To use the Virtual Environment, activate it by:

        conda activate HelloWorld

    Anything installed now will be specific to the project HelloWorld

3.  Exit the Virtual Environment

        conda deactivate
