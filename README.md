# rs-gate-server
rust gate server

```shell
# 生成 entity 模块文件
sea-orm-cli generate entity -s public --with-serde both --model-extra-attributes 'serde(rename_all = "camelCase")' --date-time-crate chrono -o ./src/entity
```