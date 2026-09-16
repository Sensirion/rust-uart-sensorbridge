Developer Build Guide
=====================

This guide provides instructions for developers building, testing, and contributing to `rust-uart-sensorbridge` and its Python bindings.

Prerequisites
-------------

Before building the project, ensure you have the following installed:

- **Rust toolchain** (1.75+ or later): Install via `rustup <https://rustup.rs/>`_.
- **Python** (3.11 or later) with the ``venv`` module.
- **Git**

Setting Up the Python Development Environment
---------------------------------------------

It is strongly recommended to use a Python virtual environment to isolate dependencies.

1. **Create and activate a virtual environment:**

   On Linux / macOS:

   .. code-block:: bash

      python3 -m venv .venv
      source .venv/bin/activate

   On Windows (PowerShell):

   .. code-block:: powershell

      python -m venv .venv
      .\.venv\Scripts\Activate.ps1

   On Windows (Command Prompt):

   .. code-block:: bat

      python -m venv .venv
      .\.venv\Scripts\activate.bat

2. **Install development dependencies:**

   .. code-block:: bash

      pip install --upgrade pip
      pip install maturin pytest ruff
      pip install -r docs/requirements.txt

Building and Testing the Rust Library
-------------------------------------

To compile the Rust crate:

.. code-block:: bash

   # Debug build
   cargo build

   # Release build
   cargo build --release

To run the Rust test suite (including unit tests, protocol mock tests, and firmware image tests):

.. code-block:: bash

   cargo test

To check code formatting and run the Rust linter (Clippy):

.. code-block:: bash

   cargo fmt --check
   cargo clippy --all-targets --all-features -- -D warnings

Building and Testing the Python Extension
-----------------------------------------

The Python bindings use `PyO3 <https://pyo3.rs/>`_ and `Maturin <https://www.maturin.rs/>`_.

1. **Build and install into active virtual environment:**

   .. code-block:: bash

      maturin develop

2. **Run Python tests:**

   .. code-block:: bash

      pytest python_tests

3. **Run Python code quality checks:**

   .. code-block:: bash

      ruff check .
      ruff format --check .

4. **Build distribution wheels and sdist:**

   .. code-block:: bash

      maturin build --release

Building Sphinx Documentation
-----------------------------

To build the HTML documentation using Sphinx:

1. Ensure your virtual environment is active and the package is installed:

   .. code-block:: bash

      maturin develop
      pip install -r docs/requirements.txt

2. Build the documentation:

   On Linux / macOS:

   .. code-block:: bash

      cd docs
      make html

   On Windows:

   .. code-block:: bat

      cd docs
      make.bat html

3. The generated documentation will be available in ``docs/_build/html/index.html``.
