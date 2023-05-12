# 镜像切换工具 

## 安装

```shell
   #第一步安装
   cargo install cargo-zproxy
   #第二步骤初始化代理
   cargo zproxy init
   #第三步骤初始化代理
   cargo zproxy use  --source tuna  
   # 更多仓库镜像，请使用cargo zproxy list 查看可使用的镜像地址
```

## 如何恢复

```shell

   #恢复官方镜像只需要执行
   cargo zproxy def

```

## 自定义代理镜像地址。满足需要再特定添加特定的内网镜像源情况

```shell
   # 自定义添加镜像地址 例如
   cargo zproxy  add   --name "本地源"  --source  "locality" --url "http://127.0.0.1/crates.io-index"
  #切换本地源
  cargo zproxy  use --source locality 
```

## 内置索引镜像

目前内置-Github官网索引地址

``` shell
cargo zproxy list #查看已配置镜像源地址内容
crates.io-index 官网地址
清华，
北京外国语，
浙江大学，
哈尔滨工业大学等镜像
更多使用cargo zproxy list查看
```

## 计划实现功能

- [x] 1： 配置资源初始化

- [ ]    3:   list 所有可用镜像地址--按照网络延
  迟排序
  
 - - [x] list 所有可用镜像地址
   - [ ] 按照网络延迟排序

- [ ] 4:   auto 自动选择最优镜像

- [ ] 5:   use  --source xxx /default/auto/1

    - [x] 设置默认为creates镜像
    - [x] 支持切换内置的镜像配置

- [x] 6:   add  手动设置镜像地址
        -source  xxx  -url=xxxx

- [ ] 7:   sync 地址备份git仓库地址

    

## 指令列表
```shell
  cargo zproxy  init              执行本地初始化
  cargo zproxy  auto              评估网络延迟并自动切换到最优的镜像
  cargo zproxy  use <source>        切换为要使用的镜像
  cargo zproxy  list              查看当前所有配置的镜像资源地址
  cargo zproxy  add  -source<XX> --name<XX> -url xxxx
                                  添加一个镜像进本地配置文件
  cargo zproxy  sync              同步镜像配置文件
                  --repo <XX>     设置私有配置仓库
                  --push          同步镜像配置文件
  cargo zproxy  version           查看当前版本
```
