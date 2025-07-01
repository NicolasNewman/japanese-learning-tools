Submodule for building [mecab-ipadic-neologd](https://github.com/neologd/mecab-ipadic-neologd)

```sh
git submodule init
git submodule update
```

## Dependencies
### Ubuntu
```sh
sudo aptitude install mecab libmecab-dev mecab-ipadic-utf8 git make curl xz-utils file
```

### Fedora/RHEL/CentOS
```sh
sudo dnf install mecab mecab-devel mecab-ipadic git make curl xz file
```

### Other

See mecab-ipadic-neologd's [README](https://github.com/neologd/mecab-ipadic-neologd?tab=readme-ov-file#examples) for other distros