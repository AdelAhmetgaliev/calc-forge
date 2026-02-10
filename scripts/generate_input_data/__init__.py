import os
from os.path import dirname, abspath, normpath, join, exists

ROOT_DIR = dirname(abspath(__file__))
DATA_DIR = normpath(join(ROOT_DIR, "..", "..", "data"))

if not exists(DATA_DIR):
    os.makedirs(DATA_DIR)
