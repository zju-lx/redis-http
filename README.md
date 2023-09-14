# redis-http
 
支持 `ping` `get` `set` `del` 三种指令

## Quik Start:

1. `cargo run --bin redis-http`
2. `cargo run --bin server`
3. 打开浏览器

    * `Ping` URL: `127.0.0.1:3000/ping`

        返回 `PONG` 则连接成功

    * `Get` URL:`127.0.0.0:3000/get/:key`

        `:key` 替换成要查询的 key

        若存在则返回查询结果，若不存在则返回 `not found`

    * `Set` URL: `127.0.0.1:3000/set`

        在 key 和 value 栏填入要创建的键值对，Subscribe！

        返回 `set ok` 则插入成功

    * `Del` URL: `127.0.0.1:3000/del`

        在 key 栏填入要删除的键值对，Subscribe！

        返回`ok`则插入成功