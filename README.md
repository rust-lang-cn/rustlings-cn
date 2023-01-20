# rustlings 🦀❤️

你好，欢迎来到 `rustlings`。这个项目包括一些让你熟悉阅读和编写 Rust 代码的小练习。这包括阅读和响应编译器信息！

_...寻找老版本的基于web的 Rustlings 版本？ 试试 [这里](https://github.com/rust-lang/rustlings/tree/rustlings-1)_

或者，对于第一次学习 Rust 的人，这里还有一些其它的资源：

- [Rust 程序设计语言](https://doc.rust-lang.org/book/index.html) - 学习 Rust 最全面的资源，但有时有点偏理论性。你将和 Rustlings 一起使用它。
- [Rust 程序设计语言（中文版）](https://rustwiki.org/zh-CN/book/) - 学习 Rust 最全面的资源，但有时有点偏理论性。你将和 Rustlings 一起使用它。
- [通过例子学 Rust](https://doc.rust-lang.org/rust-by-example/index.html) - 通过解决小练习来学习 Rust！ 这很像 `rustlings`，但是它是在线的。
- [通过例子学 Rust（中文版）](https://rustwiki.org/zh-CN/rust-by-example/) - 通过解决小练习来学习 Rust！ 这很像 `rustlings`，但是它是在线的。

## 开始

_注意：如果你使用 MacOS，请确认你已经通过输入 `xcode-select --install` 安装了 Xcode 以及它的开发者工具_

_注意：如果你使用 Linux，请确认你已经安装了 gcc。Deb: `sudo apt install gcc`。 Yum: `sudo yum -y install gcc`。_

你需要安装 Rust。你可以通过 https://rustup.rs 获取。这同时将会安装 Cargo，其为 Rust 的包/项目管理器。

## MacOS/Linux

只需要执行:

```bash
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash
# 或者如果你希望在其它路径上安装
curl -L https://raw.githubusercontent.com/rust-lang/rustlings/main/install.sh | bash -s mypath/
```

这将会安装 Rustlings 并让你可以使用 `rustlings` 命令，运行它开始吧！

### Nix
基本上：在最新 tag 处克隆本仓库，最后运行 `nix develop` 或者 `nix-shell`。

```bash
# 在 https://github.com/rust-lang/rustlings/releases/latest 获取最新版本 (编辑此文档时是 5.3.0)
git clone -b 5.3.0 --depth 1 https://github.com/rust-lang/rustlings
# 译者注：为了降低工作量，该仓库只维护main分支，所以直接克隆本仓库的主分支就行
git clone -b main --depth 1 https://github.com/rust-lang-cn/rustlings-cn.git
cd rustlings
# 如果 nix 版本号 > 2.3
nix develop
# 如果 nix 版本号 <= 2.3
nix-shell
```

## Windows

在 Powershell （使用管理员身份运行），设置 `ExecutionPolicy` 为 `RemoteSigned`：

```ps1
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

然后，你可以运行：

```ps1
Start-BitsTransfer -Source https://raw.githubusercontent.com/rust-lang/rustlings/main/install.ps1 -Destination $env:TMP/install_rustlings.ps1; Unblock-File $env:TMP/install_rustlings.ps1; Invoke-Expression $env:TMP/install_rustlings.ps1
```

安装 Rustlings。与 MacOS/Linux 一样，安装完成之后你将可以访问 `rustlings` 命令。请注意，它在 PowerShell 中表现最好，任何其它的终端都有可能会出错。

如果你收到权限被拒绝的信息，你可能必须在防病毒软件中排除克隆 Rustlings 的目录。

## 浏览器

[在 Repl.it 运行](https://repl.it/github/rust-lang-cn/rustlings-cn)

[![在 Gitpod 打开](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/rust-lang-cn/rustlings-cn)

## 手动安装

基本上：在最新 tag 处克隆该仓库，执行 `cargo install --path .` 。

```bash
# 在 https://github.com/rust-lang/rustlings/releases/latest 获取最新版本 (编辑此文档时是 5.3.0)
git clone -b 5.3.0 --depth 1 https://github.com/rust-lang/rustlings
# 译者注：为了降低工作量，该仓库只维护main分支，所以直接克隆本仓库的主分支就行
git clone -b main --depth 1 https://github.com/rust-lang-cn/rustlings-cn.git
cd rustlings
cargo install --force --path .
```

如果遇到任何安装错误，请确保你的工具链是最新的。运行以下命令来获取最新版本：

```bash
rustup update
```

然后，与之前说的一样，运行 `rustlings` 以开始。

## 做练习

练习按主题排序，你可以在子目录 `rustlings/exercises/<topic>` 下看到。每个主题有一个额外的 README 文件，内含一些资源以帮助你开始该主题。我们强烈推荐你在开始练习之前查看这些资源。

任务是简单的。大多数练习包含一个错误导致它们无法被编译，需要你来修复它！一些练习也作为测试运行，但是 rustlings 处理他们的方式是一样的。要以推荐的顺序运行练习，请执行：

```bash
rustlings watch
```

这将会尝试按预定顺序（我们认为最适合新手的顺序）验证每个练习的完成情况。它也会在你每次更改 `exercises/` 路径下的文件时自动运行。如果你想要它只运行一次，可以使用：

```bash
rustlings verify
```

这与 watch 功能做的事情一样，但是会在运行完成后自动退出。

如果你想要以你自己的顺序进行，或者只想验证一个练习，可以运行：

```bash
rustlings run myExercise1
```

或者简单地使用下面的命令来运行课程中下一个未解决的问题：

```bash
rustlings run next
```

如果遇到困难，可以运行以下命令来获取练习的提示：

```bash
rustlings hint myExercise1
```

你也可以通过以下指令获得下一个未解决问题的提示：

```bash
rustlings hint next
```

可以通过以下指令检查你的进度：

```bash
rustlings list
```

## 自我测评

在每几个部分之后，会有一个测验来测试你在这些部分的所学内容。这些测试可以在 `exercises/quizN.rs` 中找到。

## 启用 `rust-analyzer`

运行 `rustlings lsp` 命令来在工程的根目录生成 `rust-project.json`，这会允许 [rust-analyzer](https://rust-analyzer.github.io/) 解析每个练习。

## 更进一步

完成 Rustlings 后，充分利用你的新知识！通过构建你自己的项目、为 Rustlings 或你找到的其它开源项目做贡献，继续练习你的 Rust 技能。

## 卸载 Rustlings

如果你想要从你的系统中移除 Rustlings，需要进行两个步骤。首先，你需要删除安装脚本为你创建的练习文件夹：

```bash
rm -rf rustlings # 或者你自定义的文件夹名字，如果你选择或重命名了它
```

然后，由于 Rustlings 是通过 `cargo install` 安装的，因此可以合理地假设你也可以使用 Cargo 将其删除，事实确实如此。运行 `cargo uninstall` 以删除 `rustlings` 的二进制可执行文件：

```bash
cargo uninstall rustlings
```

现在你应该已经搞定了！

## 贡献

详见 [CONTRIBUTING.md](./CONTRIBUTING.md).

关于 Rustlings 的以开发为中心的讨论在 [Zulip 上的 Rust 项目](https://rust-lang.zulipchat.com) 的 [**rustlings** 流](https://rust-lang.zulipchat.com/#narrow/stream/334454-rustlings) 中发生。如果你有想法或建议，请随时在那里开启一个新线程！

## 贡献者 ✨

感谢 [AUTHORS.md](./AUTHORS.md) 中列出的优秀人员 🎉
