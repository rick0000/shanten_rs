from setuptools import setup
from setuptools_rust import Binding, RustExtension

setup(name='shanten',
      version='0.1.0',
      rust_extensions=[
          RustExtension('shanten',
                        'Cargo.toml', binding=Binding.PyO3)],
      # rust extensions are not zip safe, just like C-extensions.
      zip_safe=False)