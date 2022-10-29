from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

# SEE: https://github.com/PyO3/setuptools-rust/tree/main/examples/namespace_package
setup(
  name="rusty_graph",
  version="1.0",
  # rust_extensions=[RustExtension("src/lib.rs", binding=Binding.PyO3)], 
  rust_extensions=[RustExtension("rusty_graph.rg", "Cargo.toml", binding=Binding.PyO3)],
  package_dir = {'': 'src'},
  packages=['rusty_graph'],#find_packages,
  zip_safe=False  # rust extensions are not zip safe, just like C-extensions.
)