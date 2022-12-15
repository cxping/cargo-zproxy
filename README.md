# 镜像切换工具 -- 暂未完成，无法使用

## 计划实现功能

- [ ]    1：default ：默认create-index

- [ ]    2:   list 所有可用镜像地址---按照网络延
迟排序

- [ ] 3:   auto 自动选择最优镜像

- [ ] 4:   use xxx /default/auto/1

- [ ]  5:   add  手动设置镜像地址
        -name  xxx  -url=xxxx

- [ ]  6:   sync 地址备份git仓库地址

- [ ]  7：

## 指令列表
```shell
  cargo zproxy  init              执行本地初始化
  cargo zproxy  auto              评估网络延迟并自动切换到最优的镜像
  cargo zproxy  use <name>        切换为要使用的镜像
  cargo zproxy  list              查看当前所有配置的镜像资源地址
  cargo zproxy  add  -source<XX> --name<XX> -url xxxx
                                  添加一个镜像进本地配置文件
  cargo zproxy  sync              同步镜像配置文件
                  --repo <XX>         设置私有配置仓库
                  --push          同步镜像配置文件
  cargo zproxy  version           查看当前版本
```

