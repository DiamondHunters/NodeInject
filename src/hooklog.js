//this file is a little example of how to use injecting ability
//to hook a function and change its behavior

// JUST FOR LEARNING PURPOSES, DON'T USE THIS TO CRACK SOFTWARE


http = require("http")

function log(str) {
    http.get('http://127.0.0.1:3000/log?str=' + str, res => {
    }).on('error', err => {
        console.log('Error: ', err.message);
    });
}

log = console.log;
console.log = log
let validator = {
    set: function (target, key, value) {
        if (key === 'log') {
            log('console.log override blocked');
            return;
        }
        target[key] = value;
    }
}

let proxy = new Proxy(console, validator);
console = proxy
