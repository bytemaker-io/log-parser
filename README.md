# Log Loader

Log Loader is a command-line program designed to load log files, parse them, and upload the data to ElasticSearch. This tool simplifies the process of handling log data for analysis and storage.

## Usage

To use Log Loader, follow the command structure below:

```bash
launcher [OPTIONS] --file-name <FILE_NAME>
```

## Help
```bash
launcher --help
```
## Build from source
install rust, please read https://forge.rust-lang.org/infra/other-installation-methods.html
```bash
git clone https://github.com/bytemaker-io/log-parser.git && cd log-parser
cargo build --release
```
## Example:
It supports multiple file names as well. We build it with multiple threads to handle multiple files at the same time.
the defalut file directory is `./log_file/` and you also can specify the file directory by `--p or --file-path <FILE_PATH>` option.
```bash
./launcher --file-names test_log.txt
./launcher --file-names test_log.txt test-log2.txt
./launcher --file-names test_log.txt test-log2.txt --p /home/user/log_files/
```





