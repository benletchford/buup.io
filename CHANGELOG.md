# Changelog

## [0.14.0](https://github.com/benletchford/buup/compare/v0.13.0...v0.14.0) (2025-05-02)


### Features

* add default test input for transformers ([9f2790d](https://github.com/benletchford/buup/commit/9f2790de2040e7f1de091180c8d80f1e92d8b941))
* display default transformation when input is empty ([2a62620](https://github.com/benletchford/buup/commit/2a62620950f4de9661f0d467ce8538c0a177e7e3))


### Bug Fixes

* bump versions to skip over 0.14.0 ([e212b73](https://github.com/benletchford/buup/commit/e212b7386936c3a6b7568f60b0d1feccf5c1998f))
* hardcode gzip_decompress default value ([9f5ec2e](https://github.com/benletchford/buup/commit/9f5ec2eb76c883ce9a33087b209f31a1bf4f51bc))
* implement default_test_input and fix test failures ([5507a81](https://github.com/benletchford/buup/commit/5507a81e389ce7fae8920ce4939b4d1a8633d8d3))
* **web:** improve output textarea behavior ([cfa58db](https://github.com/benletchford/buup/commit/cfa58db36a8a36ae4d051ca9b35bc394e11b92d6))

## [0.13.0](https://github.com/benletchford/buup/compare/v0.12.0...v0.13.0) (2025-04-30)


### Features

* add sha1hash transformer ([627f604](https://github.com/benletchford/buup/commit/627f604280a5e225886a3e0026cf6fb14f3d83d4))

## [0.12.0](https://github.com/benletchford/buup/compare/v0.11.0...v0.12.0) (2025-04-30)


### Features

* add Gzip compress and decompress transformers ([59f7202](https://github.com/benletchford/buup/commit/59f72021043a69504d3b06220ad34241b07399dd))
* **compression:** refactor deflate/gzip and improve robustness ([86ed94b](https://github.com/benletchford/buup/commit/86ed94b59e72a82c38030265032ba75b208bfb9e))


### Bug Fixes

* **web:** improve mobile layout with svh units and height adjustments ([aa16afe](https://github.com/benletchford/buup/commit/aa16afe45dfa273738fbbb43f6dfa02507b76289))

## [0.11.0](https://github.com/benletchford/buup/compare/v0.10.1...v0.11.0) (2025-04-29)


### Features

* add deflate (de)/compress transformers ([a598774](https://github.com/benletchford/buup/commit/a5987741f3c0e2233edb382f8f927fe0933e0e06))

## [0.10.1](https://github.com/benletchford/buup/compare/v0.10.0...v0.10.1) (2025-04-29)


### Bug Fixes

* **ci:** install libglib2.0-dev for workspace builds ([44f79c2](https://github.com/benletchford/buup/commit/44f79c2a91bd51bb89afb1fce913436e26dce30e))

## [0.10.0](https://github.com/benletchford/buup/compare/v0.9.0...v0.10.0) (2025-04-28)


### Features

* add JWT Decoder transformer ([179bb6a](https://github.com/benletchford/buup/commit/179bb6ab671c0b334bc49edfec9568b5a4be027f))

## [0.9.0](https://github.com/benletchford/buup/compare/v0.8.0...v0.9.0) (2025-04-28)


### Features

* Add UUID5 transformer with RFC4122 namespace support ([18ea243](https://github.com/benletchford/buup/commit/18ea24313ab1326aeaf1d966a7fbe5816177c2d2))

## [0.8.0](https://github.com/benletchford/buup/compare/v0.7.0...v0.8.0) (2025-04-28)


### Features

* add LineSorter and UniqueLines transformers ([b3d4634](https://github.com/benletchford/buup/commit/b3d463426a9af1c375d962609bed43a56a0c7ea6))
* add whitespace remover, line number adder/remover transformers ([86a5476](https://github.com/benletchford/buup/commit/86a5476cdb7956ba9f99b190c590e21c2282095d))

## [0.7.0](https://github.com/benletchford/buup/compare/v0.6.0...v0.7.0) (2025-04-28)


### Features

* add uuid, text stats, url parser, and slugify transformers ([589c302](https://github.com/benletchford/buup/commit/589c3022c1c90ddedaf4af5f8e2342c2d138d8f6))

## [0.6.0](https://github.com/benletchford/buup/compare/v0.5.0...v0.6.0) (2025-04-28)


### Features

* integrate dependency-free CLI into core crate ([f6752de](https://github.com/benletchford/buup/commit/f6752dee63cd25205d5f8bbd92cb0eb2722c148f))

## [0.5.0](https://github.com/benletchford/buup/compare/v0.4.0...v0.5.0) (2025-04-27)


### Features

* add morse code encoder/decoder transformers ([dc8b87d](https://github.com/benletchford/buup/commit/dc8b87d907c1365f783f46440c5358d3468fa69b))

## [0.4.0](https://github.com/benletchford/buup/compare/v0.3.0...v0.4.0) (2025-04-27)


### Features

* Add AsciiToHex and HexToAscii transformers ([6f3b2b8](https://github.com/benletchford/buup/commit/6f3b2b82dc75f5543329f1290965b6270ed4c9b2))


### Bug Fixes

* **web:** Improve responsiveness and scrolling on small screens ([22c0df4](https://github.com/benletchford/buup/commit/22c0df46388a3ff72e96fdf4e654c01db2b43800))

## [0.3.0](https://github.com/benletchford/buup/compare/v0.2.0...v0.3.0) (2025-04-27)


### Features

* **web:** Add version and git hash to footer ([8e01dcd](https://github.com/benletchford/buup/commit/8e01dcd704b4b1e0129be9644f6b23b44d58d792))


### Bug Fixes

* Use div_ceil in base64_encode capacity calculation ([fa33f2c](https://github.com/benletchford/buup/commit/fa33f2c9f5760f1e75cd557859b6299ee992d06c))

## [0.2.0](https://github.com/benletchford/buup/compare/v0.1.0...v0.2.0) (2025-04-27)


### Features

* Add binary encode and decode transformers ([20f66e8](https://github.com/benletchford/buup/commit/20f66e839b7ef64e97693b669047b2166e4bed99))
* **ci:** integrate readme update and gh-pages into release workflow ([e3a62aa](https://github.com/benletchford/buup/commit/e3a62aa4bf6f0520d477d02ae0f74f2affa7e7d4))

## [0.1.0](https://github.com/benletchford/buup/compare/v0.0.1...v0.1.0) (2025-04-27)


### Features

* Add integer base conversion transformers ([e607669](https://github.com/benletchford/buup/commit/e6076696f6f5ad198be6b64cddc379fb1f5ab0de))
* add ROT13 transformer ([20f3955](https://github.com/benletchford/buup/commit/20f39559daf08479c5fa72583290284afaf6ec48))
