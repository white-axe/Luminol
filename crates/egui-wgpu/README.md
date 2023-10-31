> [!NOTE]
> This is Luminol's modified version of egui-wgpu. The original version is dual-licensed under MIT and Apache 2.0.
>
> To merge changes from upstream into this crate, first perform these one-time setup steps in your local repository:
>
> ```bash
> # Add a new remote named egui that tracks the upstream egui repository
> git remote add -f --no-tags egui https://github.com/emilk/egui
>
> # Add a new local branch named subtree/egui-wgpu/base based on the
> # egui master branch (don't push it to GitHub)
> git branch subtree/egui-wgpu/base egui/master
> ```
>
> Then, fetch and merge the upstream changes (these steps have to be run again every time you want to merge):
>
> ```bash
> git fetch egui
>
> # Merge upstream changes into the subtree/egui-wgpu/base branch
> git checkout subtree/egui-wgpu/base
> git merge egui/master
>
> # Create or update a local branch named subtree/egui-wgpu/split that contains
> # only the commits that modified the upstream crates/egui-wgpu directory,
> # and also create an empty merge commit in the subtree/egui-wgpu/base branch
> # that keeps track of when Git subtree was last used so Git doesn't have to
> # re-check the entire egui history again the next time you do this
> git subtree split --rejoin -P crates/egui-wgpu -b subtree/egui-wgpu/split
>
> # Checkout the branch you want to merge into (e.g. dev)
> git checkout dev
>
> # Squash merge the changes from subtree/egui-wgpu/split into the
> # crates/egui-wgpu directory in this branch
> git subtree merge --squash -P crates/egui-wgpu subtree/egui-wgpu/split
> ```
>
> The process of handling merge conflicts during the `git subtree merge` step is the same as for regular merges.

# egui-wgpu

[![Latest version](https://img.shields.io/crates/v/egui-wgpu.svg)](https://crates.io/crates/egui-wgpu)
[![Documentation](https://docs.rs/egui-wgpu/badge.svg)](https://docs.rs/egui-wgpu)
![MIT](https://img.shields.io/badge/license-MIT-blue.svg)
![Apache](https://img.shields.io/badge/license-Apache-blue.svg)

This crates provides bindings between [`egui`](https://github.com/emilk/egui) and [wgpu](https://crates.io/crates/wgpu).

This was originally hosted at https://github.com/hasenbanck/egui_wgpu_backend
