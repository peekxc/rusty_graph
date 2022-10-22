from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

setup(
  name="rusty_graph",
  version="1.0",
  # rust_extensions=[RustExtension("src/lib.rs", binding=Binding.PyO3)], 
  rust_extensions=[RustExtension("rusty_graph.rusty_graph", "Cargo.toml")],
  package_dir = {'': 'src'},
  packages=['rusty_graph'],#find_packages,
  # install_requires=["cffi"],
  # setup_requires=["cffi"],
  zip_safe=False  # rust extensions are not zip safe, just like C-extensions.
)