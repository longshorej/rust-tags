# rustags

[![Crates.io](https://img.shields.io/crates/v/rust-tags.svg?style=flat-square)](https://crates.io/crates/rust-tags)
[![Crates.io](https://img.shields.io/crates/d/rust-tags.svg?style=flat-square)](https://crates.io/crates/rust-tags)

**This project is a work-in-progress.**

This is a HTML templating library for the [Rust Programming Language](https://www.rust-lang.org/en-US/). It's strongly inspired by [Scalatags](https://github.com/lihaoyi/scalatags/).

It differs from other templating libraries in that your documents are simply defined as plain function calls. This, among other things, allows you to safely construct documents that are free from XSS vulerabilities.

One other advantage to this approach is that features such as partial templates, etc. are easily expressed using Rust, having all of its power available.

## Example

```rust
use rust_tags::core::*;
use rust_tags::tags*;
use rust_attributes::*;

let frag = html(&[
    head(&[tags::title(&["My Blog".into()])]),
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

println!(frag.data);
```


## Author

Jason Longshore <nospam@hence.todo.com>

## License

Copyright (C) 2018 Jason Longshore (https://longshore.info/).

Licensed under the Apache License, Version 2.0 (the "License"); you may not use this project except in compliance with the License. You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0.

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
