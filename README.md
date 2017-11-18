# Gitstat

```
                   _ __      __       __ 
             ___ _(_) /____ / /____ _/ /_
            / _ `/ / __(_-</ __/ _ `/ __/
            \_, /_/\__/___/\__/\_,_/\__/ 
           /___/     
```

Perform a fast scan of your entire filesystem for Git repositories with uncommitted work.

## Quick Start

Via Homebrew (mac):

    brew tap jondot/tap
    brew install gitstat

Or download a recent binary from [releases](https://github.com/jondot/gitstat/releases).

You can now let `gitstat` scan one or more root paths:

    gitstat ~/projects ~/experiments

Within seconds, you should have a listing of 'dirty' Git repositories.


## Automate

`gitstat` is fast. You can set up a cron job that runs it on a critical root path (say, `~/work`) to remind you to finish your work or commit work-in-progress bits to a branch.


# Contributing

Fork, implement, add tests, pull request, get my everlasting thanks and a respectable place here :).


### Thanks:

To all [Contributors](https://github.com/jondot/gitstat/graphs/contributors) - you make this happen, thanks!


# Copyright

Copyright (c) 2017 [Dotan Nahum](http://gplus.to/dotan) [@jondot](http://twitter.com/jondot). See [LICENSE](LICENSE.txt) for further details.
