# rust-tags

[![Crates.io](https://img.shields.io/crates/v/rust-tags.svg?style=flat-square)](https://crates.io/crates/rust-tags)
[![Crates.io](https://img.shields.io/crates/d/rust-tags.svg?style=flat-square)](https://crates.io/crates/rust-tags)
[![Travis-ci.org](https://travis-ci.org/longshorej/rust-tags.svg?branch=master)](https://travis-ci.org/longshorej/rust-tags)

**This project is a work-in-progress.**

This is an HTML templating library for the [Rust Programming Language](https://www.rust-lang.org/en-US/). It's strongly inspired by [Scalatags](https://github.com/lihaoyi/scalatags/).

It differs from other templating libraries in that your documents are simply defined as plain function calls. This, among other things, allows you to safely construct documents that are free from XSS vulerabilities.

One other advantage to this approach is that features such as partial templates, etc. are easily expressed using Rust, having all of its power available.

## Example

```rust
extern crate rust_tags;

use rust_tags::tags::*;
use rust_tags::tags::title;
use rust_tags::attributes::*;

fn main() {
    let frag = html(&[
        head(&[title(&["My Blog".into()])]),
        body(&[
            div(&[
                "Jason Longshore".into(),

                hr(&[]),

                // note that the hello world is escaped

                a(&[href("#"), "My Blog <hello world />".into()]),

                br(),
                br()
            ])
        ])
    ]);

    println!("{}", frag.data);
}
```

## Projects That Use `rust-tags`

* [jason-longshore](https://github.com/longshorej/jason-longshore)

## Release Notes

##### 0.3.1 2018-02-18

Fix a bug in attribute value escaping.

##### 0.3.0 2018-02-18

Initial usable release.

## Releasing

1) Upgrade version in `Cargo.toml`
5) Update the Release Notes in `README.md`
2) Commit changes
3) Create and push a tag: ```git tag v<version>; git push origin v<version>```
4) Release on crates.io: ```cargo publish```

## Author

Jason Longshore <hello@jasonlongshore.com>

## License

Copyright (C) 2018 Jason Longshore (https://longshore.info/).

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this project except in compliance with the License. You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
