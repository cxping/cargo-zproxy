# 镜像切换工具 

目前内置-Github官网索引地址

```
crates.io-index 官网地址
清华，北京外国语，浙江大学，哈尔滨工业大学等镜像更多使用cargo zproxy list查看
```

## 计划实现功能

- [x] 1： 配置资源初始化

- [ ]    3:   list 所有可用镜像地址--按照网络延
  迟排序
  
 - - [x] list 所有可用镜像地址
   - [ ] 按照网络延迟排序

- [ ] 4:   auto 自动选择最优镜像

- [ ] 5:   use xxx /default/auto/1

    - [x] 设置默认为creates镜像
    - [x] 支持切换内置的镜像配置

- [ ] 6:   add  手动设置镜像地址
        -name  xxx  -url=xxxx

- [ ] 7:   sync 地址备份git仓库地址

    

## 指令列表
```shell
  cargo zproxy  init              执行本地初始化
  cargo zproxy  auto              评估网络延迟并自动切换到最优的镜像
  cargo zproxy  use <name>        切换为要使用的镜像
  cargo zproxy  list              查看当前所有配置的镜像资源地址
  cargo zproxy  add  -source<XX> --name<XX> -url xxxx
                                  添加一个镜像进本地配置文件
  cargo zproxy  sync              同步镜像配置文件
                  --repo <XX>     设置私有配置仓库
                  --push          同步镜像配置文件
  cargo zproxy  version           查看当前版本
```
