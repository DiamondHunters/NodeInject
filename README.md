# NodeInject
An inject tools for injecting js code into electron application

Please follow DMCA when using this code
### How it works:

1. unpack `node_modules.asar` package (in `./resources`)
2. write `hook.js`  into `raven` package directory (raven will be required at the early stage of startup in some application)
3. modify `index.js` of `raven`,injecting require of `hook.js`

> Currently using embedded javascript file (`hooklog.js`)

usage:

1. modify `hook.js` if you need
2. use `cargo build` to make  executable
3. Move the program to the electron application directory
4. run


### Compatibility test

- Windows / Typora 1.4.8          PASSED
- Ubuntu / Typora 1.4.7             PASSED

Since macos may adopt different packaging methods and webkit as the execution environment, this tool does not support applications under macos.

### Examples

https://github.com/DiamondHunters/NodeInject_Hook_example
