### My Find

* `./find {target directory} {expression}` : Find all the files and its path in the target directory which match the expression;
    * If `{target directory}` is `null`,the target directory will be current directory.
    * If both `{target directory}` and `{expression}` are `null`, it will list all the files of the current directory.
    * If the  `{expression}` is `-v` or `--verbose`, it will list all the files of the target directory.

* `./find -d {target directory 1} {target directory 2} ... -r {expression 1} {expression 2} ...` : Find all the files in every target directory satisfy any of the expressions.
    * `-d` can't be `null` if you have more than one target directory, and must be `null` if `{target directory}` is `null`.
    * `-r` can't be `null` if you have more than one target directory.
    * If `{target directory}` is `null`,the target directory will be current directory.

* `{expression}` can't be `null` expect `./find`.
