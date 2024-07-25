<a name="v1.0.1"></a>

### v1.1.3 (2024-07-25)

#### Bug Fixes

 - <csr-id-927c987a5e3383a2cd2eb499fc8378f381cd2526/> last number not identified in an isolated value pair
   I am using the Lexer to parse json chunks that I get in stdin one line at a time.
   
   I appreciate my use case isn't the intended one for this lib as all the tests deal with fully formed json objects and nowhere it states that this lib will work with chunks. But since it works perfectly for my use case, I thought others might benefit from this fix. But also feel free to ignore it.
   
   This PR fixes a problem where the last number value won't be identified as a number because numbers don't have a terminator like strings. In normal circumstances the next token would serve as the terminator (comma, curly brackets, etc) but if the number line is the last one in an json object, the string just ends and the object closing curly brace is in the next line so the number gets ignored.
   
   This fix checks if the last processed token was Number and returns the token type accordingly.
   
   Happy to add any further improvements or additional tests that would be beneficial

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 1635 calendar days.
 - 1635 days passed between releases.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Avoid using the 'alpha' suffix ([`1b9c1d7`](https://github.com/byron/json-tools/commit/1b9c1d73019d28cd41e9bace2cc6c8fc50f1f264))
    - Last number not identified in an isolated value pair ([`927c987`](https://github.com/byron/json-tools/commit/927c987a5e3383a2cd2eb499fc8378f381cd2526))
    - Isolated value pairs fix ([`2b47444`](https://github.com/byron/json-tools/commit/2b47444a5c412c20222cb0a4f006238e2f4f71cc))
    - Optimize includes ([`3fe2df2`](https://github.com/byron/json-tools/commit/3fe2df2c6778045745ef16b492de4ca7ca0bfd37))
    - Add new badges ([`3c9fb9e`](https://github.com/byron/json-tools/commit/3c9fb9e4a3b67787d7421ac88e9057d4374a7601))
    - Add clippy and cargo-fmt lints ([`6129e80`](https://github.com/byron/json-tools/commit/6129e801822d85ae1a6734a67b72d5a34040c556))
    - Create rust.yml ([`ee18ffa`](https://github.com/byron/json-tools/commit/ee18ffa1be5d171e32a815b513e662201d29d2ab))
    - (cargo-release) start next development iteration 1.1.3-alpha.0 ([`0668d4a`](https://github.com/byron/json-tools/commit/0668d4a17955b4453cb70e79cc87300c17208af5))
</details>

### v1.1.2 (2020-02-01)

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 0 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Add readme, too ([`88f67ea`](https://github.com/byron/json-tools/commit/88f67eaa52e368d79e6c8f44faf7c18d0965ddd7))
    - (cargo-release) start next development iteration 1.1.2-alpha.0 ([`bfd6675`](https://github.com/byron/json-tools/commit/bfd667502ddbf94d7ed466609fb984f0c8d2b53b))
</details>

### v1.1.1 (2020-02-01)

#### Chore

 - <csr-id-79ae6780059d4682f18f5bcf1bcf5e868a847c3a/> v1.0.1

#### Documentation

 - <csr-id-bcaaebb89c731423913e11d601d332090c5438b6/> README misses BufferType argument to Lexer
 - <csr-id-cc24d29a41b326401441991d47a6a31fe6d31999/> change description

#### Bug Fixes

 - <csr-id-df37dc8dd602a2e1c8656d33baafd4dcc4db2993/> handle more string escapes
 - <csr-id-e1afcbcd3fcee79594b557c03e7cdb59f0409c8c/> handle numbers with exponents
 - <csr-id-123555099611d98c935459c996d700eee3f33859/> syntax error
 - <csr-id-067e93aa4c401cedb1a6134735ac0fcb7ca0e1f9/> only run benches on nightly

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 16 commits contributed to the release over the course of 1124 calendar days.
 - 1124 days passed between releases.
 - 7 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Use criterion for benchmarks ([`2be30e9`](https://github.com/byron/json-tools/commit/2be30e9adedb2012329d149606cf23b31dccc875))
    - Improve readme ([`0b036d6`](https://github.com/byron/json-tools/commit/0b036d673c55020cf221af8c026ed583a16131bc))
    - Clippy ([`08057a6`](https://github.com/byron/json-tools/commit/08057a67612a9dd86a894369f4deeb64204f0049))
    - Simplification and modernization ([`4f168d4`](https://github.com/byron/json-tools/commit/4f168d4b8384f7406bf860b682ee2d820febc7cc))
    - Merge pull request #12 from FauxFaux/patch-1 ([`520763f`](https://github.com/byron/json-tools/commit/520763f2b00893cfb9be56fc8ce28c1bd5396ba3))
    - README misses BufferType argument to Lexer ([`bcaaebb`](https://github.com/byron/json-tools/commit/bcaaebb89c731423913e11d601d332090c5438b6))
    - Cut 1.1.0 ([`5be61b2`](https://github.com/byron/json-tools/commit/5be61b2e27bb3304f54718cbf59eda2a58fb34d3))
    - Merge pull request #11 from heycam/num-str-fixes ([`df6d401`](https://github.com/byron/json-tools/commit/df6d401e0fed48c3d34ef1eec53a71adb0524b1f))
    - Handle more string escapes ([`df37dc8`](https://github.com/byron/json-tools/commit/df37dc8dd602a2e1c8656d33baafd4dcc4db2993))
    - Handle numbers with exponents ([`e1afcbc`](https://github.com/byron/json-tools/commit/e1afcbcd3fcee79594b557c03e7cdb59f0409c8c))
    - Add crates badge ([`b53e873`](https://github.com/byron/json-tools/commit/b53e8736d12916e8a2be6e08d62e8fc91f899adb))
    - Apply latest rustfmt ([`0750473`](https://github.com/byron/json-tools/commit/075047304aa2827001b67c9b0b1fb8789dddcce3))
    - Syntax error ([`1235550`](https://github.com/byron/json-tools/commit/123555099611d98c935459c996d700eee3f33859))
    - Only run benches on nightly ([`067e93a`](https://github.com/byron/json-tools/commit/067e93aa4c401cedb1a6134735ac0fcb7ca0e1f9))
    - V1.0.1 ([`79ae678`](https://github.com/byron/json-tools/commit/79ae6780059d4682f18f5bcf1bcf5e868a847c3a))
    - Change description ([`cc24d29`](https://github.com/byron/json-tools/commit/cc24d29a41b326401441991d47a6a31fe6d31999))
</details>

### v1.1.0 (2019-02-08)

This adds support for Number tokens with exponents and String tokens with the full range of escapes that are allowed. 

<a name="v1.0.1"></a>

### v1.0.1 (2017-01-02)

Changed the headline of the crate to be more descriptive.


<a name="v1.0.0"></a>

### v1.0.0 (2017-01-02)

#### Improvements

* **travis:**  more rust versions and no travis-cargo ([fd4cd367](https://github.com/Byron/json-tools/commit/fd4cd36794eba4de0a6ebbe000ddddc8ce5c96d4))

#### Other

 - <csr-id-fd4cd36794eba4de0a6ebbe000ddddc8ce5c96d4/> more rust versions and no travis-cargo
   The latter is only useful for doc-uploading, yet
   we don't need that anymore in the time of docs.rs

#### Chore

 - <csr-id-fb0758eb1883a6ed794bdef6e6b3b36d6a3e5f33/> format everything
 - <csr-id-030f5801c8813ad8b3bd13cf15d2eeec26ef927e/> remove do-not-edit note
   A left-over of the original
   
   [skip ci]

#### Bug Fixes

* **benchmark:**  remove old code ([f969498a](https://github.com/Byron/json-tools/commit/f969498a5f17b4d0f260e7e1e0ea46007abe75f0))
 - <csr-id-f969498a5f17b4d0f260e7e1e0ea46007abe75f0/> remove old code
   It didn't compile anymore, and also probably wasn't
   really testing us anyway.

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 8 commits contributed to the release over the course of 559 calendar days.
 - 604 days passed between releases.
 - 4 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Format everything ([`fb0758e`](https://github.com/byron/json-tools/commit/fb0758eb1883a6ed794bdef6e6b3b36d6a3e5f33))
    - Remove old code ([`f969498`](https://github.com/byron/json-tools/commit/f969498a5f17b4d0f260e7e1e0ea46007abe75f0))
    - More rust versions and no travis-cargo ([`fd4cd36`](https://github.com/byron/json-tools/commit/fd4cd36794eba4de0a6ebbe000ddddc8ce5c96d4))
    - Merge pull request #10 from cmr/master ([`da0d2c8`](https://github.com/byron/json-tools/commit/da0d2c8b2941c99f0bb87862c586eb8953fe2d92))
    - Relicense to dual MIT/Apache-2.0 ([`853e77a`](https://github.com/byron/json-tools/commit/853e77aa5c98070e8b099e928f983c6e09c48fc5))
    - Merge pull request #8 from jnicholls/master ([`9303648`](https://github.com/byron/json-tools/commit/9303648c595428035e1e1b673eaaf1ae1ae180bf))
    - Rust 1.x stable compatibility fix. ([`d2f6a43`](https://github.com/byron/json-tools/commit/d2f6a43f8b57992104a9288a730a7cb038111eff))
    - Remove do-not-edit note ([`030f580`](https://github.com/byron/json-tools/commit/030f5801c8813ad8b3bd13cf15d2eeec26ef927e))
</details>

<csr-unknown>
<a name="v0.3.0"></a><csr-unknown/>

### v0.3.0 (2015-05-09)

#### Features

* **iterator-ext**  more fun with Token-Iterators ([15dc5c5f](https://github.com/Byron/json-tools/commit/15dc5c5f9a3e153afdbeca3e4d741288c2f77111), closes [#3](https://github.com/Byron/json-tools/issues/3))



<a name="v0.1.1"></a>

#### Refactor

 - <csr-id-de49700411db02cf91ff8de92862ba4b42891208/> use `IntoItertor`
   We use `IntoIterator` in place of `Iterator` which should provide more
   flexibility when feeding the Lexer. However, in our tests, we can't
   actually use that, unfortunately, due to the consuming semantics (and
   literals cannot be consumed ... ).
   
   However, it's a nice proof of concept and doesn't hurt.

#### Chore

 - <csr-id-0f540c6713090f00fbcbd91111ee21876e2c94e3/> v0.3.0

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V0.3.0 ([`0f540c6`](https://github.com/byron/json-tools/commit/0f540c6713090f00fbcbd91111ee21876e2c94e3))
    - Use `IntoItertor` ([`de49700`](https://github.com/byron/json-tools/commit/de49700411db02cf91ff8de92862ba4b42891208))
</details>

### v0.2.0 (2015-05-09)

#### Chore

 - <csr-id-f40b44c1354830f9f14a7b673b16007649855599/> v0.2.0

#### New Features

 - <csr-id-15dc5c5f9a3e153afdbeca3e4d741288c2f77111/> more fun with Token-Iterators
   * attaches constructor utilities to all `Iterator<Item=Token>` to ease
     using them. It's similar to (former) `IteratorExt` which would put
     `chain()` and `nth` for instance.

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - V0.2.0 ([`f40b44c`](https://github.com/byron/json-tools/commit/f40b44c1354830f9f14a7b673b16007649855599))
    - More fun with Token-Iterators ([`15dc5c5`](https://github.com/byron/json-tools/commit/15dc5c5f9a3e153afdbeca3e4d741288c2f77111))
</details>

### v0.1.1 (2015-05-08)

#### Chore

 - <csr-id-0d125f22f194f27a35e9adec6ad12e67d85a80c9/> set v0.1.1

#### Bug Fixes

* **token-reader**  offset-map for target buffer ([57768da1](https://github.com/Byron/json-tools/commit/57768da16ca72505a8424e012d4525e9bf583fd6), closes [#6](https://github.com/Byron/json-tools/issues/6))
 - <csr-id-57768da16ca72505a8424e012d4525e9bf583fd6/> offset-map for target buffer
   Previously we would keep writing the first bytes of our destination
   buffer, as we wouldn't compute any offset at all. Now we produce slices
   of exactly the right size, and could could verify that this is working.

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 2 commits contributed to the release.
 - 2 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Set v0.1.1 ([`0d125f2`](https://github.com/byron/json-tools/commit/0d125f22f194f27a35e9adec6ad12e67d85a80c9))
    - Offset-map for target buffer ([`57768da`](https://github.com/byron/json-tools/commit/57768da16ca72505a8424e012d4525e9bf583fd6))
</details>

<csr-unknown>
<a name="v0.1.0"></a><csr-unknown/>

### v0.0.1 (2015-05-06)

#### Chore

 - <csr-id-f3b4120dbd2d22ee9b14d87bc516260f9f197052/> initial commit

#### Commit Statistics

<csr-read-only-do-not-edit/>

 - 1 commit contributed to the release.
 - 1 commit was understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

#### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Initial commit ([`f3b4120`](https://github.com/byron/json-tools/commit/f3b4120dbd2d22ee9b14d87bc516260f9f197052))
</details>

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

### Refactor

 - <csr-id-08ad49b70b868e9e21a6331bbbb6143d1a748821/> optimize buffer usage
   * only push characters when we have a buffer and are dealing with
     strings or numbers.
   * added some performance tests to show the difference. We are not quite
     back at 280MB/s, but down to 220MB/s for the optimal/Span case.
     The Bytes case goes down to 137MB/s.
 - <csr-id-e924f03a8eadc5ab4b0a6dba7d3b51d33853c724/> use enum for state
   Instead of using many different variables for handling the state, we
   use an enumeration. That way, we don't unnecessarily initialize memory
   that will never be used, and lower our requirements for stack space.
 - <csr-id-0a7e5c7e5fe0cff55d4f241ecc437e2eb860fe29/> separarte lexer and filters
   * `FilterNull` now has its own module
   * `Lexer` and friends have their own module
 - <csr-id-dd40f6eb53d79397474e2dbdcf66ddc1c6bc3663/> string value fast-path
   Technically it's not a fast-path, but it makes the code more uniform
   and easier to understand.
   It will be the default template for all other lexing we do,
   for instance when implementing booleans.

### Other

 - <csr-id-75181ff6a52863da26dc83f2997cfa46edc8756c/> update to match current state
   We also anticipate pretty-printing, which technically isn't there yet.
 - <csr-id-a5e3c3d767b91ab420cfbedd81fc1232d82cddf9/> high-speed serialize tests
   Producers are optimized for performance to show exactly how fast a
   span token producer is compared to a bytes token producer.
   
   That way, performance improvements can exactly be quantified.
 - <csr-id-8c5e9f2598888a095d4cfed5681c71394ee5a86b/> added benchmarks
   Under optimial conditions (source string is known) we remove null values
   from 144MB/s, fully streamed we do 104MB/s.
   
   An acceptable result, considering the unoptimized buffer handling,
   using a deque might improve this a lot.
   
   However, we manage to retrieve invalid tokens, which we have to handle
   somehow, and also don't expect here.
 - <csr-id-431f051d15adebd365a9bf379dbf61ac8037658b/> make it general
   * What's formerly known as NullFilter can now filter out all key-value
     pairs with a given key-value type.
     That way, null can be filtered, as well as numbers, for example.
   * Made certain dual-branch matches an if-clause, which moved everything
     furthe to the left again, while making the code easier to read.
 - <csr-id-d5a694d11a783fb65e6388a442cbb66784a1bb3a/> operate on u8 instead of char
   This improved throughput from 230MB/s to 280MB/s, which is quite
   worth the while. In case of json, only the values within Strings are
   actually potentially unicode, everything else is not
 - <csr-id-43a1119fe6e2f40b9c434e1ae80c60929a4192bd/> added benchmark
   Also renamed test-related files to match their purpose a bit better.
 - <csr-id-d4782c8e83853d689bf84b08fe48f713059f9c78/> benchmark and string_value tests
   * verify escaping in string values works
   * added more complex benchmark, lexing 415MB/s
   * tested unclosed string value

### New Features

 - <csr-id-458928ddd45c85f2588ffcdb7d5efcccb2c88eb1/> machine serialization works
   In a first version, we show that serialization without any space
   works as expected according to very first tests.
   
   More tests have to be conducted to be sure.
 - <csr-id-96dac09649fa27752851dbdc4babfe12a1f5db63/> infrastructure setup
   * added `TokenReader` type with basic API
   * improve existing filter tests to verify TokenReader operation
 - <csr-id-a3e72b50fc09a73fc0ea8d4bb70ae10085386d71/> support for `Buffer` enum
   It cuts our speed in half, currently, but allows to choose between
   high-speed Span mode and half-speed buffer mode.
   That way, all applications I see can be catered with best performance.
 - <csr-id-97adcb85b7c7198edcba49bdd56a246d701dd05d/> initial implementation
   It is known to not work in all cases, i.e. it can only take one
   key-value pair at a time (no consecutive ones), but besides that
   is a pretty optimal implementation (even though it aint a pretty one).
   
   However, the test still fails, we match nothing for some reason.
   Must be evaluated later.
 - <csr-id-f952f087685a6b86aa2f4a45869f9ddc3d5a7578/> number lexing
   Even though our performance dropped from 332MB/s to 274MB/s, we are
   happy as the implementation cleaned up our span handling considerably
   and thus made the code even more maintainable.
   Cleaning up the span handling also made it faster, i.e. slowest number
   parsing was at 232MB/s.
 - <csr-id-fb94ea9c4193a169cc54fed4c032f02dda302145/> filtering iterator infrastucture
   * FilterNull frame would allow implementing lexical token pattern
     matching to remove null values.
 - <csr-id-97ae90808aabf8699e951700f59cb11e2cd38376/> true and false value lexing
   * including test for both
   * refactored test case for null value easily test booleans as well.
 - <csr-id-dc2f9a2500440c73eace8ccdac35a25897611d5b/> null value lexing
   * including tests for normal null values and invalid ones
 - <csr-id-e9b60721104361a2ace0d592551050f35a7d8178/> string value parsing
   Including test
 - <csr-id-f66ea5ffe879b7dc9e551abd401be9bd3439662a/> datastructures and basic tests
   The tests still fail as the actual lexer implementation is still to be
   done.

### Documentation

 - <csr-id-077fe4132ed92470b5f3317abf0b1114eb5a2bba/> added remaining documentation
   I believe it's best not to add redundant information into the library
   docs, but instead refer to the tests and benchmarks.
 - <csr-id-270f57c36892d2e1e1f1ae16773c2538235d2ddd/> state why numbers won't be lexed
   Also it's not required to solve our actual problem.
 - <csr-id-e9cc19e62d27f6d181ea070f64d8518656070367/> *usage* added

### Chore

 - <csr-id-ffb3c71e64c23a32b47afbb8210d8418b1a11fb2/> clog config + changelog
 - <csr-id-32cd37ba1f3fc341d8ceeaabeeda41404af0ebef/> also run benchmarks
 - <csr-id-b5ab5a15af66b20773e3a93f58a8e8f9652ba7b6/> set GH_TOKEN
   ... instead of TOKEN
 - <csr-id-5287268c034627fcfddd965722f4de4c54afe4e7/> with doc-upload
   Never worked for me, but let's try it one more time.
 - <csr-id-dc439b655a98a4267cf187af68c06058ae582341/> no nightly please
   As we didn't set the package unstable
 - <csr-id-dbf42ec42ca1bb2e5838dc36e466c90104447da9/> added secret
   minor format adjustment

#### Improvements

* **lexer**  operate on u8 instead of char ([d5a694d1](https://github.com/Byron/json-tools/commit/d5a694d11a783fb65e6388a442cbb66784a1bb3a))
* **null-filter**  make it general ([431f051d](https://github.com/Byron/json-tools/commit/431f051d15adebd365a9bf379dbf61ac8037658b))
* **README**  update to match current state ([75181ff6](https://github.com/Byron/json-tools/commit/75181ff6a52863da26dc83f2997cfa46edc8756c))

### Bug Fixes

* **null-filter**
  *  removed possible overflow ([50c9f81c](https://github.com/Byron/json-tools/commit/50c9f81cdf83a1868d4456f74e1b78354fb6cd98))
  *  proper comma handling ([321fa592](https://github.com/Byron/json-tools/commit/321fa59258c431007c48a6c0155b0bf9c4f50780))
  *  handle consecutive null values ([96e20e65](https://github.com/Byron/json-tools/commit/96e20e656e63c827afa1942c1d09ad76544e9776))
  *  minor fix to make it work ([e489bffa](https://github.com/Byron/json-tools/commit/e489bffa8eb16741ad91c6b17ae5552dd9ad3a3a))
*  proper comma handling ([321fa592](https://github.com/Byron/json-tools/commit/321fa59258c431007c48a6c0155b0bf9c4f50780))
*  handle consecutive null values ([96e20e65](https://github.com/Byron/json-tools/commit/96e20e656e63c827afa1942c1d09ad76544e9776))
*  minor fix to make it work ([e489bffa](https://github.com/Byron/json-tools/commit/e489bffa8eb16741ad91c6b17ae5552dd9ad3a3a))
 - <csr-id-e489bffa8eb16741ad91c6b17ae5552dd9ad3a3a/> minor fix to make it work
   It's still far from perfect, but a good proof of concept
 - <csr-id-96e20e656e63c827afa1942c1d09ad76544e9776/> handle consecutive null values
   With this in place, we handle null value filtering pretty well, as
   the tests indicate too.
   
   However, we may still leave a trailing comma in non-null values which
   could be a problem and thus shouldn't be done !
 - <csr-id-321fa59258c431007c48a6c0155b0bf9c4f50780/> proper comma handling
   * Added support for leading `,` characters, which have to be removed
     conditionally.
   * Added tests to verify this works in valid streams, and even invalid
     ones.
 - <csr-id-50c9f81cdf83a1868d4456f74e1b78354fb6cd98/> removed possible overflow
   Previously it was possible to over-allocate memory by feeding us lots
   of `,` characters. This was alleviated by allowing a one-token
   look-ahead (implemented through put-back).
 - <csr-id-1d57bc923cb34b6daf1105691b700815a82cc0c1/> handle whitespace at end of source
   Previously we would consider such whitespace invalid. Now we explictly
   set the invalid state, which is ... explicit = better :) !
   
   Added we added a test to show this actually works.

### Commit Statistics

<csr-read-only-do-not-edit/>

 - 35 commits contributed to the release over the course of 2 calendar days.
 - 2 days passed between releases.
 - 35 commits were understood as [conventional](https://www.conventionalcommits.org).
 - 0 issues like '(#ID)' were seen in commit messages

### Commit Details

<csr-read-only-do-not-edit/>

<details><summary>view details</summary>

 * **Uncategorized**
    - Added remaining documentation ([`077fe41`](https://github.com/byron/json-tools/commit/077fe4132ed92470b5f3317abf0b1114eb5a2bba))
    - Clog config + changelog ([`ffb3c71`](https://github.com/byron/json-tools/commit/ffb3c71e64c23a32b47afbb8210d8418b1a11fb2))
    - Update to match current state ([`75181ff`](https://github.com/byron/json-tools/commit/75181ff6a52863da26dc83f2997cfa46edc8756c))
    - Handle whitespace at end of source ([`1d57bc9`](https://github.com/byron/json-tools/commit/1d57bc923cb34b6daf1105691b700815a82cc0c1))
    - High-speed serialize tests ([`a5e3c3d`](https://github.com/byron/json-tools/commit/a5e3c3d767b91ab420cfbedd81fc1232d82cddf9))
    - Added benchmarks ([`8c5e9f2`](https://github.com/byron/json-tools/commit/8c5e9f2598888a095d4cfed5681c71394ee5a86b))
    - Machine serialization works ([`458928d`](https://github.com/byron/json-tools/commit/458928ddd45c85f2588ffcdb7d5efcccb2c88eb1))
    - Infrastructure setup ([`96dac09`](https://github.com/byron/json-tools/commit/96dac09649fa27752851dbdc4babfe12a1f5db63))
    - Make it general ([`431f051`](https://github.com/byron/json-tools/commit/431f051d15adebd365a9bf379dbf61ac8037658b))
    - Optimize buffer usage ([`08ad49b`](https://github.com/byron/json-tools/commit/08ad49b70b868e9e21a6331bbbb6143d1a748821))
    - Support for `Buffer` enum ([`a3e72b5`](https://github.com/byron/json-tools/commit/a3e72b50fc09a73fc0ea8d4bb70ae10085386d71))
    - Operate on u8 instead of char ([`d5a694d`](https://github.com/byron/json-tools/commit/d5a694d11a783fb65e6388a442cbb66784a1bb3a))
    - Removed possible overflow ([`50c9f81`](https://github.com/byron/json-tools/commit/50c9f81cdf83a1868d4456f74e1b78354fb6cd98))
    - Proper comma handling ([`321fa59`](https://github.com/byron/json-tools/commit/321fa59258c431007c48a6c0155b0bf9c4f50780))
    - Handle consecutive null values ([`96e20e6`](https://github.com/byron/json-tools/commit/96e20e656e63c827afa1942c1d09ad76544e9776))
    - Added benchmark ([`43a1119`](https://github.com/byron/json-tools/commit/43a1119fe6e2f40b9c434e1ae80c60929a4192bd))
    - Minor fix to make it work ([`e489bff`](https://github.com/byron/json-tools/commit/e489bffa8eb16741ad91c6b17ae5552dd9ad3a3a))
    - Initial implementation ([`97adcb8`](https://github.com/byron/json-tools/commit/97adcb85b7c7198edcba49bdd56a246d701dd05d))
    - Number lexing ([`f952f08`](https://github.com/byron/json-tools/commit/f952f087685a6b86aa2f4a45869f9ddc3d5a7578))
    - Use enum for state ([`e924f03`](https://github.com/byron/json-tools/commit/e924f03a8eadc5ab4b0a6dba7d3b51d33853c724))
    - Separarte lexer and filters ([`0a7e5c7`](https://github.com/byron/json-tools/commit/0a7e5c7e5fe0cff55d4f241ecc437e2eb860fe29))
    - Filtering iterator infrastucture ([`fb94ea9`](https://github.com/byron/json-tools/commit/fb94ea9c4193a169cc54fed4c032f02dda302145))
    - State why numbers won't be lexed ([`270f57c`](https://github.com/byron/json-tools/commit/270f57c36892d2e1e1f1ae16773c2538235d2ddd))
    - True and false value lexing ([`97ae908`](https://github.com/byron/json-tools/commit/97ae90808aabf8699e951700f59cb11e2cd38376))
    - Also run benchmarks ([`32cd37b`](https://github.com/byron/json-tools/commit/32cd37ba1f3fc341d8ceeaabeeda41404af0ebef))
    - Set GH_TOKEN ([`b5ab5a1`](https://github.com/byron/json-tools/commit/b5ab5a15af66b20773e3a93f58a8e8f9652ba7b6))
    - With doc-upload ([`5287268`](https://github.com/byron/json-tools/commit/5287268c034627fcfddd965722f4de4c54afe4e7))
    - No nightly please ([`dc439b6`](https://github.com/byron/json-tools/commit/dc439b655a98a4267cf187af68c06058ae582341))
    - String value fast-path ([`dd40f6e`](https://github.com/byron/json-tools/commit/dd40f6eb53d79397474e2dbdcf66ddc1c6bc3663))
    - Null value lexing ([`dc2f9a2`](https://github.com/byron/json-tools/commit/dc2f9a2500440c73eace8ccdac35a25897611d5b))
    - Benchmark and string_value tests ([`d4782c8`](https://github.com/byron/json-tools/commit/d4782c8e83853d689bf84b08fe48f713059f9c78))
    - String value parsing ([`e9b6072`](https://github.com/byron/json-tools/commit/e9b60721104361a2ace0d592551050f35a7d8178))
    - *usage* added ([`e9cc19e`](https://github.com/byron/json-tools/commit/e9cc19e62d27f6d181ea070f64d8518656070367))
    - Added secret ([`dbf42ec`](https://github.com/byron/json-tools/commit/dbf42ec42ca1bb2e5838dc36e466c90104447da9))
    - Datastructures and basic tests ([`f66ea5f`](https://github.com/byron/json-tools/commit/f66ea5ffe879b7dc9e551abd401be9bd3439662a))
</details>

<csr-unknown>
lexer  handle whitespace at end of source (https://github.com/Byron/json-tools/commit/1d57bc923cb34b6daf1105691b700815a82cc0c11d57bc92)<csr-unknown/>

