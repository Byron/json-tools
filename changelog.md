<a name="v1.0.1"></a>
### v1.0.1 (2017-01-02)

Changed the headline of the crate to be more descriptive.


<a name="v1.0.0"></a>
## v1.0.0 (2017-01-02)


#### Improvements

* **travis:**  more rust versions and no travis-cargo ([fd4cd367](https://github.com/Byron/json-tools/commit/fd4cd36794eba4de0a6ebbe000ddddc8ce5c96d4))

#### Bug Fixes

* **benchmark:**  remove old code ([f969498a](https://github.com/Byron/json-tools/commit/f969498a5f17b4d0f260e7e1e0ea46007abe75f0))



<a name="v0.3.0"></a>
## v0.3.0 (2015-05-09)


#### Features

* **iterator-ext**  more fun with Token-Iterators ([15dc5c5f](https://github.com/Byron/json-tools/commit/15dc5c5f9a3e153afdbeca3e4d741288c2f77111), closes [#3](https://github.com/Byron/json-tools/issues/3))



<a name="v0.1.1"></a>
## v0.1.1 (2015-05-08)


#### Bug Fixes

* **token-reader**  offset-map for target buffer ([57768da1](https://github.com/Byron/json-tools/commit/57768da16ca72505a8424e012d4525e9bf583fd6), closes [#6](https://github.com/Byron/json-tools/issues/6))



<a name="v0.1.0"></a>
## v0.1.0 (2015-05-08)


#### Features

* **lexer**
  *  support for `Buffer` enum ([a3e72b50](https://github.com/Byron/json-tools/commit/a3e72b50fc09a73fc0ea8d4bb70ae10085386d71))
  *  number lexing ([f952f087](https://github.com/Byron/json-tools/commit/f952f087685a6b86aa2f4a45869f9ddc3d5a7578))
  *  filtering iterator infrastucture ([fb94ea9c](https://github.com/Byron/json-tools/commit/fb94ea9c4193a169cc54fed4c032f02dda302145))
  *  true and false value lexing ([97ae9080](https://github.com/Byron/json-tools/commit/97ae90808aabf8699e951700f59cb11e2cd38376))
  *  null value lexing ([dc2f9a25](https://github.com/Byron/json-tools/commit/dc2f9a2500440c73eace8ccdac35a25897611d5b))
  *  string value parsing ([e9b60721](https://github.com/Byron/json-tools/commit/e9b60721104361a2ace0d592551050f35a7d8178))
  *  datastructures and basic tests ([f66ea5ff](https://github.com/Byron/json-tools/commit/f66ea5ffe879b7dc9e551abd401be9bd3439662a))
* **null-filter**  initial implementation ([97adcb85](https://github.com/Byron/json-tools/commit/97adcb85b7c7198edcba49bdd56a246d701dd05d))
* **token-reader**
  *  machine serialization works ([458928dd](https://github.com/Byron/json-tools/commit/458928ddd45c85f2588ffcdb7d5efcccb2c88eb1))
  *  infrastructure setup ([96dac096](https://github.com/Byron/json-tools/commit/96dac09649fa27752851dbdc4babfe12a1f5db63))

#### Improvements

* **lexer**  operate on u8 instead of char ([d5a694d1](https://github.com/Byron/json-tools/commit/d5a694d11a783fb65e6388a442cbb66784a1bb3a))
* **null-filter**  make it general ([431f051d](https://github.com/Byron/json-tools/commit/431f051d15adebd365a9bf379dbf61ac8037658b))
* **README**  update to match current state ([75181ff6](https://github.com/Byron/json-tools/commit/75181ff6a52863da26dc83f2997cfa46edc8756c))

#### Bug Fixes

* **null-filter**
  *  removed possible overflow ([50c9f81c](https://github.com/Byron/json-tools/commit/50c9f81cdf83a1868d4456f74e1b78354fb6cd98))
  *  proper comma handling ([321fa592](https://github.com/Byron/json-tools/commit/321fa59258c431007c48a6c0155b0bf9c4f50780))
  *  handle consecutive null values ([96e20e65](https://github.com/Byron/json-tools/commit/96e20e656e63c827afa1942c1d09ad76544e9776))
  *  minor fix to make it work ([e489bffa](https://github.com/Byron/json-tools/commit/e489bffa8eb16741ad91c6b17ae5552dd9ad3a3a))
* **lexer**  handle whitespace at end of source ([1d57bc92](https://github.com/Byron/json-tools/commit/1d57bc923cb34b6daf1105691b700815a82cc0c1))



