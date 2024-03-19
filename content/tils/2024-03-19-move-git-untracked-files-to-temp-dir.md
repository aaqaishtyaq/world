+++
title = "Move untracked git files to another directory"
date = "2024-03-19"
+++


I had a lot of files in a git repository, which I didn't want to add in `git`, So to remove all of them
to a temporary directory I used a collection of Unix commands.

```bash
git status --porcelain | awk '{print $2}' | xargs mv -t ../scratch
```
