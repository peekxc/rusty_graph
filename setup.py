from setuptools import setup, find_packages
from setuptools_rust import Binding, RustExtension

setup(
  name="rusty_graph",
  version="1.0",
  rust_extensions=[RustExtension("src/lib.rs", binding=Binding.PyO3)], 
  #packages=["src/rusty_graph"],
  package_dir = {'': 'src'},
  packages=['rusty_graph'],#find_packages
  zip_safe=False  # rust extensions are not zip safe, just like C-extensions.
)